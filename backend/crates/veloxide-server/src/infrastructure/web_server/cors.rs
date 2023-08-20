use hyper::{
    header::{ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE},
    http::{HeaderName, HeaderValue},
    Method,
};
use tower_http::cors::CorsLayer;

use super::configuration::FRONTEND_CLIENT_ORIGIN_ENV_VAR;

pub fn new_cors_layer() -> CorsLayer {
    let frontend_client_origin: String =
        dotenvy::var(FRONTEND_CLIENT_ORIGIN_ENV_VAR).unwrap_or("http://localhost:5173".to_string());
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers(vec![
            CONTENT_TYPE,
            ACCEPT,
            ACCEPT_ENCODING,
            AUTHORIZATION,
            "x-grpc-web".parse::<HeaderName>().unwrap(),
            "x-user-agent".parse::<HeaderName>().unwrap(),
        ])
        .allow_origin(frontend_client_origin.parse::<HeaderValue>().unwrap())
}
