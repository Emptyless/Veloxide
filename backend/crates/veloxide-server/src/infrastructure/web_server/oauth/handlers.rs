use std::collections::HashMap;

use crate::infrastructure::web_server::oauth::handlers::auth::auth_config;
use crate::infrastructure::{cryptography::*, web_server::configuration::DEFAULT_REDIRECT_PATH};
use axum::{
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
};

use chrono::Utc;
use color_eyre::eyre::{eyre, ContextCompat};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};

use crate::infrastructure::{middleware::*, web_server::oauth::google::*};
use tower_cookies::Cookies;

pub const AUTH_TOKEN_COOKIE_NAME: &str = "auth-token";

use crate::{
    domain::{
        oauth2_state::OAuth2State, user_aggregate::User, user_repository::UserRepository,
        Oauth2StateRepository,
    },
    infrastructure::{
        auth_utils::*, repositories::OAuth2StateRepositoryImpl, repositories::UserRepositoryImpl,
    },
};

#[utoipa::path(
    get,
    tag = "Auth",
    path = "/protected",
    responses(
        (status = 200, description = "A protected route that requires authentication and authorisation"),
        (status = 403, description = "Your access was denied due to either not being authenticated or not having the correct permissions")
    )
  )]
#[tracing::instrument(ret, err)]
pub async fn protected() -> Result<&'static str, AuthError> {
    Ok("protected")
}

#[utoipa::path(
    get,
    tag = "Auth",
    path = "/login",
    responses(
        (status = 302, description = "Redirect to Google's login page")
    )
  )]
#[tracing::instrument(ret, err)]
pub async fn login(
    cookies: Cookies,
    Query(mut params): Query<HashMap<String, String>>,
    Extension(user_data): Extension<Option<UserData>>,
    Extension(oauth_client): Extension<BasicClient>,
    Extension(mut oauth2_state_repo): Extension<OAuth2StateRepositoryImpl>,
) -> Result<Redirect, AuthError> {
    if user_data.is_some() {
        return Ok(Redirect::temporary(DEFAULT_REDIRECT_PATH));
    }

    let return_url = params
        .remove("return_url")
        .unwrap_or_else(|| DEFAULT_REDIRECT_PATH.to_string());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let oauth2_state = OAuth2State::new(
        csrf_token.secret().clone(),
        pkce_verifier.secret().clone(),
        return_url,
    );

    oauth2_state_repo
        .create_state(oauth2_state)
        .await
        .map_err(|_| AuthError::StateStoreFailed)?;

    Ok(Redirect::temporary(auth_url.as_str()))
}

#[tracing::instrument(ret, err)]
pub async fn google_oauth_callback_handler(
    Query(mut params): Query<HashMap<String, String>>,
    cookies: Cookies,
    Extension(mut oauth2_state_repo): Extension<OAuth2StateRepositoryImpl>,
    Extension(mut user_repo): Extension<UserRepositoryImpl>,
    Extension(oauth_client): Extension<BasicClient>,
) -> crate::prelude::Result<impl IntoResponse> {
    let query_csrf_state = CsrfToken::new(params.remove("state").wrap_err("OAuth: without state")?);
    let code = AuthorizationCode::new(params.remove("code").ok_or(AuthError::WithoutCode)?);

    let oauth2_state: OAuth2State = oauth2_state_repo
        .get_state(query_csrf_state.secret().as_str())
        .await?;
    let crsf_state_equal = oauth2_state.csrf_state == *query_csrf_state.secret();
    if !crsf_state_equal {
        return Err(AuthError::CsrfStateMismatch.into());
    }

    let pkce_code_verifier = PkceCodeVerifier::new(oauth2_state.code_verifier.clone());

    let token_response = oauth_client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|_| eyre!("OAuth: failed to exchange code"))?;

    let access_token = token_response.access_token().secret();
    let user_info_url = GOOGLE_USERINFO_URL.to_owned() + access_token;

    let body = reqwest::get(user_info_url)
        .await
        .map_err(|_| eyre!("OAuth: failed to query userinfo"))?
        .text()
        .await
        .map_err(|_| eyre!("OAuth: received invalid userinfo"))?;

    let mut body: serde_json::Value = serde_json::from_str(body.as_str())
        .map_err(|_| eyre!("OAuth: Serde failed to parse userinfo"))?;

    tracing::info!("OAuth: userinfo: {:?}", body);

    let email = body["email"]
        .take()
        .as_str()
        .ok_or(eyre!("OAuth: Serde failed to parse email address"))?
        .to_owned();

    let verified_email = body["verified_email"]
        .take()
        .as_bool()
        .ok_or(eyre!("OAuth: Serde failed to parse verified_email"))?;

    if !verified_email {
        return Err(AuthError::EmailAddressNotVerified.into());
    }

    let get_user_result = user_repo.get_user_by_email(&email).await;

    let user = match get_user_result {
        Ok(user) => user,
        Err(_) => {
            let user = User::new(email.clone());
            user_repo.create_user(&user).await?;
            user
        }
    };

    let key = &auth_config().token_key;
    let auth_token: AuthToken = new_web_token(
        &user.email,
        Utc::now() + chrono::Duration::days(1),
        &user.token_salt.to_string(),
        key,
    )?;
    set_auth_cookie(
        &cookies,
        &auth_token.to_string(),
        Some(auth_token.expiration),
    );
    Ok(Redirect::to(oauth2_state.return_url.as_str()))
}

#[utoipa::path(
    post,
    tag = "Auth",
    path = "/logout",
    responses(
        (status = 200, description = "Logout")
    )
  )]
#[tracing::instrument(ret, err)]
pub async fn logout(cookies: Cookies) -> Result<impl IntoResponse, AuthError> {
    let _auth_token = cookies
        .get(AUTH_TOKEN_COOKIE_NAME)
        .ok_or(AuthError::AuthTokenNotFound)?
        .value()
        .to_owned();

    remove_auth_token_cookie(&cookies);

    Ok(Redirect::to(DEFAULT_REDIRECT_PATH))
}
