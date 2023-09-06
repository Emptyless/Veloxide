use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use tower_cookies::Cookies;

use super::*;
use crate::{
    domain::user_repository::UserRepository,
    infrastructure::{
        auth_utils::*, cryptography::*, grpc::auth_grpc_service::UserView,
        repositories::UserRepositoryImpl,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthConfiguration {
    pub policy_server_url: String,
    pub authz_enabled: bool,
    pub auth_token_configuration: AuthTokenConfiguration,
    pub google_oauth_enabled: bool,
    pub microsoft_oauth_enabled: bool,
}

pub fn auth_config() -> &'static AuthConfiguration {
    static INSTANCE: OnceLock<AuthConfiguration> = OnceLock::new();
    INSTANCE.get_or_init(AuthConfiguration::from_env)
}

impl AuthConfiguration {
    pub fn from_env() -> AuthConfiguration {
        let authz_enabled = dotenvy::var("AUTHZ_ENABLED")
            .unwrap_or("true".to_string())
            .parse()
            .expect("expected to be able to parse AUTHZ_ENABLED env var to a boolean");
        let policy_server_url = match authz_enabled {
            true => dotenvy::var("POLICY_SERVER_URL").expect("POLICY_SERVER_URL must be set"),
            false => String::new(),
        };
        let auth_token_configuration = AuthTokenConfiguration::from_env();
        let google_oauth_enabled = dotenvy::var("GOOGLE_OAUTH_ENABLED")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("expected to be able to parse GOOGLE_OAUTH_ENABLED env var to a boolean");
        let microsoft_oauth_enabled = dotenvy::var("MICROSOFT_OAUTH_ENABLED")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("expected to be able to parse MICROSOFT_OAUTH_ENABLED env var to a boolean");

        AuthConfiguration {
            policy_server_url,
            authz_enabled,
            auth_token_configuration,
            google_oauth_enabled,
            microsoft_oauth_enabled,
        }
    }
}

#[tracing::instrument(
    err,
    skip(cookies, next, user_repo, request),
    fields(
        method = %request.method(),
        uri = %request.uri(),
        version = ?request.version().clone(),
        host = ?request.headers().get("host"),
        connection = ?request.headers().get("connection"),
        sec_ch_ua = ?request.headers().get("sec-ch-ua"),
        sec_ch_ua_mobile = ?request.headers().get("sec-ch-ua-mobile"),
        sec_ch_ua_platform = ?request.headers().get("sec-ch-ua-platform"),
        upgrade_insecure_requests = ?request.headers().get("upgrade-insecure-requests"),
        user_agent = ?request.headers().get("user-agent"),
        accept = ?request.headers().get("accept"),
        sec_fetch_site = ?request.headers().get("sec-fetch-site"),
        sec_fetch_mode = ?request.headers().get("sec-fetch-mode"),
        sec_fetch_user = ?request.headers().get("sec-fetch-user"),
        sec_fetch_dest = ?request.headers().get("sec-fetch-dest"),
        referer = ?request.headers().get("referer"),
        accept_encoding = ?request.headers().get("accept-encoding"),
        accept_language = ?request.headers().get("accept-language"),
    )
)]
pub async fn mw_authenticate<B: std::fmt::Debug>(
    cookies: Cookies,
    Extension(user_repo): Extension<UserRepositoryImpl>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AuthError> {
    let user_data_result = resolve_user_data(&cookies, user_repo).await;

    if user_data_result.is_err() && !matches!(user_data_result, Err(AuthError::AuthTokenNotFound)) {
        tracing::info!("removing invalid token");
        remove_auth_token_cookie(&cookies);
    }

    if user_data_result.is_ok() {
        request.extensions_mut().insert(Some(user_data_result?));
    }
    Ok(next.run(request).await)
}

#[tracing::instrument(skip(cookies, user_repo), ret, err, level = "info")]
async fn resolve_user_data(
    cookies: &Cookies,
    user_repo: UserRepositoryImpl,
) -> Result<UserView, AuthError> {
    let token_cookie_value = get_user_token_cookie_value(cookies)?;

    let token: AuthToken = token_cookie_value
        .parse()
        .map_err(|_| AuthError::InvalidTokenFormat)?;

    let user = user_repo
        .get_user_by_email(&token.identifier)
        .await
        .map_err(|_| AuthError::FailedToGetUser)?;

    validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| AuthError::TokenValidationFailed)?;

    let new_expiration = Some(
        token.expiration
            + (auth_config()
                .auth_token_configuration
                .token_duration_minutes),
    );
    set_auth_cookie(cookies, &token.to_string(), new_expiration);

    Ok(user.into())
}

#[tracing::instrument(ret, err, level = "debug", skip(token, token_salt))]
fn validate_web_token(token: &AuthToken, token_salt: &str) -> crate::prelude::Result<()> {
    let key = &auth_config().auth_token_configuration.token_key.as_bytes();
    validate_token_signature_and_expiry(token, token_salt, key)?;
    Ok(())
}

/// Validate if the origin_token signature match what it is supposed to match.
#[tracing::instrument(ret, err, level = "debug", skip(key, origin_token, salt))]
fn validate_token_signature_and_expiry(
    origin_token: &AuthToken,
    salt: &str,
    key: &[u8],
) -> Result<(), AuthError> {
    let new_sign_b64u = token_sign_into_base64url(
        &origin_token.identifier,
        &origin_token.expiration.to_rfc3339(),
        salt,
        key,
    )?;

    if new_sign_b64u != origin_token.signature {
        return Err(AuthError::TokenSignatureNotMatching);
    }

    if origin_token.expiration < now() {
        return Err(AuthError::TokenExpired);
    }

    Ok(())
}

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

const PATH_SEPERATOR: &str = "/";

#[tracing::instrument(
    err,
    skip(cookies, next, request, headers),
    fields(
        method = %request.method(),
        uri = %request.uri(),
        version = ?request.version().clone(),
        host = ?request.headers().get("host"),
        connection = ?request.headers().get("connection"),
        sec_ch_ua = ?request.headers().get("sec-ch-ua"),
        sec_ch_ua_mobile = ?request.headers().get("sec-ch-ua-mobile"),
        sec_ch_ua_platform = ?request.headers().get("sec-ch-ua-platform"),
        upgrade_insecure_requests = ?request.headers().get("upgrade-insecure-requests"),
        user_agent = ?request.headers().get("user-agent"),
        accept = ?request.headers().get("accept"),
        sec_fetch_site = ?request.headers().get("sec-fetch-site"),
        sec_fetch_mode = ?request.headers().get("sec-fetch-mode"),
        sec_fetch_user = ?request.headers().get("sec-fetch-user"),
        sec_fetch_dest = ?request.headers().get("sec-fetch-dest"),
        referer = ?request.headers().get("referer"),
        accept_encoding = ?request.headers().get("accept-encoding"),
        accept_language = ?request.headers().get("accept-language"),
    )
)]
pub async fn mw_authorise<B>(
    cookies: Cookies,
    method: axum::http::Method,
    original_uri: axum::extract::OriginalUri,
    headers: axum::http::HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AuthError> {
    if !&auth_config().authz_enabled {
        return Ok(next.run(request).await);
    }
    let policy_server_url = &auth_config().policy_server_url;
    let auth_token_result = get_auth_token(&cookies).ok();
    let path = original_uri
        .path()
        .trim_start_matches(PATH_SEPERATOR)
        .trim_end_matches(PATH_SEPERATOR)
        .split(PATH_SEPERATOR)
        .collect::<Vec<&str>>();
    let header_hashmap: std::collections::HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let input: serde_json::Value = match auth_token_result {
        Some(auth_token) => {
            tracing::debug!(?auth_token.identifier);
            json!({
                "input": {
                "method": method.as_str(),
                "path": path,
                "user": {
                "email": auth_token.identifier.as_str(),
                "token_expiry": auth_token.expiration.to_rfc3339(),
            },
                "headers": header_hashmap
            }
            })
        }
        None => {
            json!({
                    "input": {
                    "method": method.as_str(),
                    "path": path,
                    "headers": header_hashmap
                }
            })
        }
    };

    let client = reqwest::Client::new();
    let res = client.post(policy_server_url).json(&input).send().await;
    match res {
        Ok(opa_res) => {
            let opa_res_body = match opa_res.json::<serde_json::Value>().await {
                Ok(body) => body,
                Err(_) => {
                    return Ok((
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to parse OPA response".to_string(),
                    )
                        .into_response())
                }
            };

            let allowed = opa_res_body["result"]["allow"].as_bool().unwrap_or(false);
            tracing::info!(%opa_res_body);
            if allowed {
                Ok(next.run(request).await)
            } else {
                //TODO: Replace this string the error message provided by opa if there is one
                Ok((
                    axum::http::StatusCode::FORBIDDEN,
                    "Access Denied by Policy".to_string(),
                )
                    .into_response())
            }
        }
        Err(_) => Ok((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Error in making request to OPA".to_string(),
        )
            .into_response()),
    }
}
