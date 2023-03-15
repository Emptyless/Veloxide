use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use axum::{debug_handler, Router};
use oauth2::reqwest::async_http_client;
use rand::distributions::DistString;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tracing::instrument;

use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreClaimName, CoreClaimType, CoreClient,
    CoreClientAuthMethod, CoreGrantType, CoreIdTokenClaims, CoreIdTokenVerifier, CoreJsonWebKey,
    CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm, CoreResponseMode, CoreResponseType,
    CoreSubjectIdentifierType,
};
use openidconnect::{
    AdditionalProviderMetadata, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, IssuerUrl, Nonce, ProviderMetadata, RedirectUrl, RevocationUrl, Scope,
};

// Teach openidconnect-rs about a Google custom extension to the OpenID Discovery response that we can use as the RFC
// 7009 OAuth 2.0 Token Revocation endpoint. For more information about the Google specific Discovery response see the
// Google OpenID Connect service documentation at: https://developers.google.com/identity/protocols/oauth2/openid-connect#discovery
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}
impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}
type GoogleProviderMetadata = ProviderMetadata<
    RevocationEndpointProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;

use const_format::concatcp;

use crate::prelude::Error;
use crate::state::AppState;
const SERVER_BASE_PATH: &str = "http://localhost:8080";
const GOOGLE_AUTH_CALLBACK_PATH: &str = concatcp!(SERVER_BASE_PATH, "/auth/signin/google/callback");
const GOOGLE_ISSUER_URL: &str = "https://accounts.google.com";

pub async fn new_google_openidconnect_client() -> CoreClient {
    let google_client_id = ClientId::new(
        dotenvy::var("GOOGLE_CLIENT_ID")
            .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );
    let google_client_secret = ClientSecret::new(
        dotenvy::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );
    let issuer_url = IssuerUrl::new(GOOGLE_ISSUER_URL.to_string()).expect("Invalid issuer URL");

    let provider_metadata = GoogleProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .expect("Failed to discover OpenID Provider metadata (Google)");

    let revocation_endpoint = provider_metadata
        .additional_metadata()
        .revocation_endpoint
        .clone();

    tracing::debug!(
        "Discovered Google revocation endpoint: {}",
        revocation_endpoint
    );

    // Set up the config for the Google OAuth2 process.
    CoreClient::from_provider_metadata(
        provider_metadata,
        google_client_id,
        Some(google_client_secret),
    )
    .set_redirect_uri(
        RedirectUrl::new(GOOGLE_AUTH_CALLBACK_PATH.to_string()).expect("Invalid redirect URL"),
    )
    // Google supports OAuth 2.0 Token Revocation (RFC-7009)
    .set_revocation_uri(
        RevocationUrl::new(revocation_endpoint).expect("Invalid revocation endpoint URL"),
    )
}

#[instrument(skip(state))]
pub async fn new_axum_google_auth_layer(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signin/google", get(google_login_handler))
        .route("/signin/google/callback", get(google_callback_handler))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}

#[instrument]
#[debug_handler]
pub async fn google_login_handler(State(state): State<Arc<AppState>>) -> Result<Redirect, Error> {
    let (authorize_url, csrf_state, nonce) = state
        .google_openidconnect_client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new(String::from("openid")))
        .add_scope(Scope::new(String::from("email")))
        .add_scope(Scope::new(String::from("profile")))
        .add_prompt(CoreAuthPrompt::Consent)
        .add_prompt(CoreAuthPrompt::SelectAccount)
        .url();

    println!(
        "Google OAuth2 authorization URL: {}",
        authorize_url.as_ref()
    );

    redis::cmd("SET")
        .arg(csrf_state.secret().to_string())
        .arg(nonce.secret())
        .arg("EX")
        .arg(3600)
        .query_async(&mut state.redis_client.get_tokio_connection().await?)
        .await?;
    Ok(Redirect::to(authorize_url.as_ref()))
}

#[derive(serde::Deserialize, Debug)]
pub struct IdentifyQuery {
    pub code: String,
    pub state: String,
}

#[instrument]
#[debug_handler]
pub async fn google_callback_handler(
    Query(query): Query<IdentifyQuery>,
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, Error> {
    let maybe_nonce_string: Option<String> = redis::cmd("GETDEL")
        .arg(query.state)
        .query_async(&mut state.redis_client.get_tokio_connection().await?)
        .await?;
    let nonce_string = maybe_nonce_string.ok_or(Error::InvalidState)?;
    let nonce = Nonce::new(nonce_string);

    let token_response = state
        .google_openidconnect_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .ok()
        .ok_or(Error::CodeExchangeFailed)?;
    let id_token_verifier: CoreIdTokenVerifier =
        state.google_openidconnect_client.id_token_verifier();
    let user_data: &CoreIdTokenClaims = token_response
        .extra_fields()
        .id_token()
        .ok_or(Error::NoIdToken)?
        .claims(&id_token_verifier, &nonce)?;
    let subject = user_data.subject().as_str();
    let _email = user_data.email().ok_or(Error::NoEmail)?.as_str();
    let token = rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    redis::cmd("SET")
        .arg(&token)
        .arg(subject)
        .arg(&["EX", "1209600"])
        .query_async(&mut state.redis_client.get_tokio_connection().await?)
        .await?;
    redis::cmd("SET")
        .arg(subject)
        .arg(&token)
        .arg(&["EX", "1209600"])
        .query_async(&mut state.redis_client.get_tokio_connection().await?)
        .await?;
    cookies.add(
        Cookie::build("token", token)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish(),
    );
    Ok(Redirect::to("/"))
}
