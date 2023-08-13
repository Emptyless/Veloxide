use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};

// TODO: Make these not public
pub const GOOGLE_CLIENT_ID_ENV_VAR: &str = "GOOGLE_CLIENT_ID";
pub const GOOGLE_CLIENT_SECRET_ENV_VAR: &str = "GOOGLE_CLIENT_SECRET";
pub const GOOGLE_REDIRECT_URL_ENV_VAR: &str = "GOOGLE_REDIRECT_URL"; // example: http://localhost:8080/auth/google/callback
pub const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
pub const GOOGLE_REVOCATION_URI: &str = "https://oauth2.googleapis.com/revoke";
pub const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token=";

pub fn build_google_oauth_client() -> BasicClient {
    dotenvy::dotenv().expect("Failed to read .env file");
    let client_id = dotenvy::var(GOOGLE_CLIENT_ID_ENV_VAR).expect("GOOGLE_CLIENT_ID not set");
    let client_secret =
        dotenvy::var(GOOGLE_CLIENT_SECRET_ENV_VAR).expect("GOOGLE_CLIENT_SECRET not set");
    let redirect_url =
        dotenvy::var(GOOGLE_REDIRECT_URL_ENV_VAR).expect("GOOGLE_REDIRECT_URL not set");
    let redirect_url = RedirectUrl::new(redirect_url).expect("Invalid redirect URL");
    let auth_url =
        AuthUrl::new(GOOGLE_AUTH_URL.to_string()).expect("Invalid authorization endpoint URL");
    let token_url =
        TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).expect("Invalid token endpoint URL");
    let revocation_uri = RevocationUrl::new(GOOGLE_REVOCATION_URI.to_string())
        .expect("Invalid revocation endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url)
    .set_revocation_uri(revocation_uri)
}
