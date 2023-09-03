use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};

pub const AZURE_OAUTH_ENABLED_ENV_VAR: &str = "AZURE_OAUTH_ENABLED";
const AZURE_CLIENT_ID_ENV_VAR: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET_ENV_VAR: &str = "AZURE_CLIENT_SECRET";
const AZURE_REDIRECT_URL_ENV_VAR: &str = "AZURE_REDIRECT_URL"; // example: http://localhost:8080/auth/azure/callback
const AZURE_TENANT_ID_ENV_VAR: &str = "AZURE_TENANT_ID";
const AZURE_REVOCATION_URI: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/logout";

pub fn build_azure_oauth_client() -> BasicClient {
    let client_id = dotenvy::var(AZURE_CLIENT_ID_ENV_VAR).expect("AZURE_CLIENT_ID not set");
    let client_secret =
        dotenvy::var(AZURE_CLIENT_SECRET_ENV_VAR).expect("AZURE_CLIENT_SECRET not set");
    let tenant_id = dotenvy::var(AZURE_TENANT_ID_ENV_VAR).expect("AZURE_TENANT_ID not set");
    let redirect_url =
        dotenvy::var(AZURE_REDIRECT_URL_ENV_VAR).expect("AZURE_REDIRECT_URL not set");
    let redirect_url = RedirectUrl::new(redirect_url).expect("Invalid redirect URL");
    let auth_url =
        AuthUrl::new(format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/authorize")).expect("Invalid authorization endpoint URL");
    let token_url =
        TokenUrl::new(format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token")).expect("Invalid token endpoint URL");
    let revocation_uri = RevocationUrl::new(AZURE_REVOCATION_URI.to_string())
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
