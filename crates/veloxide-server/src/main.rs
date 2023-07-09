#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

use axum::{routing::get, Extension, Router, Server};
use axum_prometheus::PrometheusMetricLayer;
use hyper::{header::CONTENT_TYPE, Method};
use presentation::ApiDoc;

use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::prelude::*;
mod error;
use dotenvy::dotenv;
mod application;
mod configuration;
mod domain;
mod prelude;
mod presentation;
mod state;

const HTTP_PORT_ENV_VAR: &str = "HTTP_PORT";
const HTTP_PORT_DEFAULT: &str = "8080";

#[tokio::main]
async fn main() -> Result<()> {
    // Load env variables
    dotenv().ok();

    // Configure logging
    tracing_log::LogTracer::builder()
        .ignore_crate("sqlx")
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("could not initialize log tracer");

    // Configure tracing
    match configuration::observability::configure_observability().await {
        Ok(_) => {
            tracing::debug!("tracing configured");
        }
        Err(err) => {
            tracing::error!("error configuring tracing: {}", err);
            return Err(err);
        }
    };

    let pool = configuration::get_db_connection().await?;
    let (cqrs, account_query) = presentation::get_bank_account_cqrs_framework(pool);

    // Configure prometheus layer for Axum
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // Configure CORS middleware for axum
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        // allow requests from any origin NOTE: This is not secure
        .allow_origin(Any);

    // Set up the GraphQL router
    let graphql_router =
        presentation::graphql::new_graphql_router(cqrs.clone(), account_query.clone()).await;

    // Set up the router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route(
            "/api/bank-accounts/:id",
            get(presentation::bank_account::query_handler)
                .post(presentation::bank_account::command_handler),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .nest("/graphql", graphql_router)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(cqrs.clone()))
                .layer(Extension(account_query.clone()))
                .layer(prometheus_layer)
                .layer(cors),
        )
        // The /health route is deliberately after the prometheus layer so that it's hits are not recorded
        .route("/health", get(|| async move { "HEALTHY" }));

    // Run the router
    let port = dotenvy::var(HTTP_PORT_ENV_VAR).unwrap_or_else(|_| HTTP_PORT_DEFAULT.to_string());
    let port = port.parse::<u16>()?;
    let address = format!("[::]:{}", port).parse().unwrap();
    Ok(Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const ENV_EXAMPLE_FILEPATH: &str = ".env.example";

    #[tokio::test]
    async fn test_http_port_default_in_env_example_is_set() {
        let load_result = dotenvy::from_filename_override(ENV_EXAMPLE_FILEPATH);
        assert_eq!(load_result.is_ok(), true);

        let http_port = dotenvy::var(HTTP_PORT_ENV_VAR);
        assert_eq!(http_port.unwrap(), HTTP_PORT_DEFAULT.to_string());
    }
}
