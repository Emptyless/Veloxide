#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

use crate::interfaces::hello::hello_world::greeter_server::GreeterServer;
use crate::interfaces::hello::MyGreeter;
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use axum_prometheus::PrometheusMetricLayer;
use axum_tonic::{NestTonic, RestGrpcService};
use hyper::Method;
use infrastructure::middleware::auth;
use infrastructure::{
    repositories, web_server,
    web_server::{consts::*, openapi::ApiDoc},
};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
use dotenvy::dotenv;
mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod prelude;

#[tokio::main]
async fn main() -> crate::prelude::Result<()> {
    color_eyre::install()?;
    //TODO: Improve this hacky workaround to make sure that the .env file is loaded properly
    let original_dir = std::env::current_dir()?;
    if !original_dir.ends_with("veloxide-server") {
        std::env::set_current_dir("backend/crates/veloxide-server")?;
        dotenv().ok();
        std::env::set_current_dir(original_dir)?;
    } else {
        dotenv().ok();
    }

    tracing_log::LogTracer::builder()
        .ignore_crate("sqlx")
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("could not initialize log tracer");

    match infrastructure::observability::configure_observability().await {
        Ok(_) => {
            tracing::debug!("tracing configured");
        }
        Err(err) => {
            tracing::error!("error configuring tracing: {}", err);
            return Err(err);
        }
    };

    let pool = infrastructure::get_db_connection().await?;

    // In a production app this should be as locked down as possible
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Bank account init
    let (bank_account_cqrs, bank_account_query) =
        interfaces::get_bank_account_cqrs_framework(pool.clone());
    let graphql_router = web_server::graphql::new_graphql_router(
        bank_account_cqrs.clone(),
        bank_account_query.clone(),
    )
    .await;
    let bank_account_routes = Router::new().route(
        "/:id",
        get(web_server::bank_account_handlers::query_handler)
            .post(web_server::bank_account_handlers::command_handler)
            .layer(
                ServiceBuilder::new()
                    .layer(Extension(bank_account_cqrs.clone()))
                    .layer(Extension(bank_account_query.clone())),
            ),
    );

    // Auth init
    let auth_config = infrastructure::middleware::auth::AuthConfiguration::init_from_env();
    let user_repository = repositories::UserRepositoryImpl::new(pool.clone());
    let oauth2_state_repository = repositories::OAuth2StateRepositoryImpl::new(pool.clone());
    let google_oauth2_client = web_server::oauth::build_google_oauth_client();
    let user_data: Option<infrastructure::middleware::UserData> = None;
    let auth_routes = Router::new()
        .route("/login", get(web_server::oauth::login))
        .route("/protected", get(web_server::oauth::protected))
        .route("/logout", post(web_server::oauth::logout))
        .route(
            "/auth/google/callback",
            get(web_server::oauth::google_oauth_callback_handler),
        );
    let api_routes = Router::new().nest("/bank-accounts", bank_account_routes);

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes)
        .merge(auth_routes)
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .nest("/graphql", graphql_router)
        .layer(axum::middleware::from_fn_with_state(
            auth_config,
            auth::mw_authorise,
        ))
        .layer(axum::middleware::from_fn(auth::mw_authenticate))
        .layer(Extension(google_oauth2_client))
        .layer(Extension(user_repository.clone()))
        .layer(Extension(oauth2_state_repository))
        .layer(Extension(user_data))
        .layer(prometheus_layer)
        .layer(cors.clone())
        .layer(CookieManagerLayer::new())
        // The /health route is deliberately after the prometheus layer so that it's hits are not recorded
        .route("/health", get(|| async move { "HEALTHY" }));

    let port = dotenvy::var(HTTP_PORT_ENV_VAR).unwrap_or_else(|_| HTTP_PORT_DEFAULT.to_string());
    let port = port.parse::<u16>()?;
    let address = format!("[::]:{}", port).parse().unwrap();
    let tonic_greeter_service = tonic_web::enable(GreeterServer::new(MyGreeter::default()));
    let grpc_router = Router::new().nest_tonic(tonic_greeter_service).layer(cors);
    let service = RestGrpcService::new(app, grpc_router);
    Ok(Server::bind(&address)
        .serve(service.into_make_service())
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
