use openidconnect::core::CoreClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub google_openidconnect_client: CoreClient,
    pub redis_client: redis::Client,
}
