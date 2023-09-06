use chrono::{DateTime, NaiveDateTime, Utc};

use time::OffsetDateTime;
use tower_cookies::{cookie::SameSite, Cookie, Cookies};

use crate::infrastructure::{cryptography::*, middleware::error::AuthError};

use super::middleware::auth::auth_config;

pub fn get_auth_token(cookies: &Cookies) -> Result<AuthToken, AuthError> {
    let auth_token = cookies
        .get(&auth_config().auth_token_configuration.cookie_name)
        .ok_or(AuthError::AuthTokenNotFound)?
        .value()
        .to_owned();
    let auth_token: AuthToken = auth_token
        .parse()
        .map_err(|_| AuthError::InvalidTokenFormat)?;
    Ok(auth_token)
}

#[tracing::instrument(level = "debug", skip_all)]
pub fn set_auth_cookie(
    cookies: &Cookies,
    token_value: &str,
    expiry: Option<chrono::DateTime<Utc>>,
) {
    let cookie_name = &auth_config().auth_token_configuration.cookie_name;
    let mut cookie = Cookie::new(cookie_name, token_value.to_owned());
    if let Some(expiry) = expiry {
        cookie.set_expires(Some(convert_to_offsetdatetime(expiry)));
    };
    cookie.set_same_site(SameSite::None);
    let domain = &auth_config().auth_token_configuration.cookie_domain;
    cookie.set_domain(domain);
    cookie.set_http_only(true);
    cookie.set_secure(auth_config().auth_token_configuration.cookie_secure);
    cookie.set_path("/");
    cookies.add(cookie);
}

pub fn convert_to_offsetdatetime(expiry: chrono::DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(expiry.timestamp())
        .expect("expected to be able to convert chrono datetime to OffsetDateTime")
}

#[tracing::instrument(level = "debug", skip_all)]
pub fn remove_auth_token_cookie(cookies: &Cookies) {
    let epoch: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::from_timestamp_opt(0, 0).expect("expected to be able to get epoch datetime"),
        Utc,
    );
    set_auth_cookie(cookies, "", Some(epoch))
}

pub fn get_user_token_cookie_value(cookies: &Cookies) -> Result<String, AuthError> {
    cookies
        .get(&auth_config().auth_token_configuration.cookie_name)
        .map(|cookie| cookie.value().to_string())
        .ok_or(AuthError::AuthTokenNotFound)
}
