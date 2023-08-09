use chrono::Utc;

use time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};

use crate::infrastructure::{cryptography::*, middleware::error::AuthError};

pub const AUTH_TOKEN_COOKIE_NAME: &str = "auth-token";

pub fn get_auth_token(cookies: &Cookies) -> Result<AuthToken, AuthError> {
    let auth_token = cookies
        .get(AUTH_TOKEN_COOKIE_NAME)
        .ok_or(AuthError::AuthTokenNotFound)?
        .value()
        .to_owned();
    let auth_token: AuthToken = auth_token
        .parse()
        .map_err(|_| AuthError::InvalidTokenFormat)?;
    Ok(auth_token)
}

#[tracing::instrument(level = "debug")]
pub fn set_auth_cookie(
    cookies: &Cookies,
    token_value: &str,
    expiry: Option<chrono::DateTime<Utc>>,
) {
    let mut cookie = Cookie::new(AUTH_TOKEN_COOKIE_NAME.to_owned(), token_value.to_owned());
    if let Some(expiry) = expiry {
        cookie.set_expires(Some(convert_to_offsetdatetime(expiry)));
    };
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
}

pub fn convert_to_offsetdatetime(expiry: chrono::DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(expiry.timestamp())
        .expect("expected to be able to convert chrono datetime to OffsetDateTime")
}

#[tracing::instrument(level = "debug")]
pub fn remove_auth_token_cookie(cookies: &Cookies) {
    let cookie = Cookie::named(AUTH_TOKEN_COOKIE_NAME);
    cookies.remove(cookie);
}

pub fn get_user_token_cookie_value(cookies: &Cookies) -> Result<String, AuthError> {
    cookies
        .get(AUTH_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string())
        .ok_or(AuthError::AuthTokenNotFound)
}
