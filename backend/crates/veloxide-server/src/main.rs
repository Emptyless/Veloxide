#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]
#![warn(clippy::all)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

use std::sync::Arc;

use crate::application::auth_service::AuthServiceImpl;
use crate::application::bank_account_application_service::BankAccountServiceImpl;
use crate::infrastructure::grpc::bank_account_grpc_service::GRpcBankAccountService;
use crate::infrastructure::{
    grpc::bank_account_service_server::BankAccountServiceServer,
    web_server::configuration::WebServerConfiguration,
};
use crate::interfaces::hello::hello_world::greeter_server::GreeterServer;
use crate::interfaces::hello::MyGreeter;
use auth_grpc_service::authentication_server::AuthenticationServer;
use auth_grpc_service::UserView;
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use axum_prometheus::PrometheusMetricLayer;
use axum_tonic::{NestTonic, RestGrpcService};
use infrastructure::grpc::auth_grpc_service;
use infrastructure::middleware::auth;
use infrastructure::web_server::graphql::new_graphql_router;
use infrastructure::{logging, observability};
use infrastructure::{repositories, web_server, web_server::openapi::ApiDoc};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
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
    let original_dir = std::env::current_dir();
    if let Ok(original_dir) = original_dir {
        if !original_dir.ends_with("veloxide-server") {
            let _ = std::env::set_current_dir("backend/crates/veloxide-server");
            dotenv().ok();
            let _ = std::env::set_current_dir(original_dir);
        } else {
            dotenv().ok();
        }
    }

    logging::configure_logging();
    observability::configure_observability()
        .expect("expected to be able to configure observability");

    let pool = infrastructure::get_db_connection().await?;

    let cors_layer = infrastructure::web_server::new_cors_layer();

    // Bank account init
    let (bank_account_cqrs, bank_account_view_repository) =
        interfaces::get_bank_account_cqrs_framework(pool.clone());
    let graphql_enabled: bool = std::env::var("GRAHQL_ENABLED")
        .unwrap_or("false".to_string())
        .parse()
        .expect("expected to be able to parse graphql_enabled as a bool");
    let bank_account_routes = Router::new().route(
        "/:id",
        get(web_server::bank_account_handlers::query_handler)
            .post(web_server::bank_account_handlers::command_handler)
            .layer(
                ServiceBuilder::new()
                    .layer(Extension(bank_account_cqrs.clone()))
                    .layer(Extension(bank_account_view_repository.clone())),
            ),
    );

    // Auth init
    let auth_config = infrastructure::middleware::auth::AuthConfiguration::from_env();
    let user_repository = repositories::UserRepositoryImpl::new(pool.clone());
    let oauth2_state_repository = repositories::OAuth2StateRepositoryImpl::new(pool.clone());
    let google_oauth2_client = web_server::oauth::build_google_oauth_client();
    let user_data: Option<UserView> = None;
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

    let mut axum_router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes)
        .merge(auth_routes)
        .route("/metrics", get(|| async move { metric_handle.render() }));

    if graphql_enabled {
        axum_router = axum_router.nest(
            "/grahql",
            new_graphql_router(
                bank_account_cqrs.clone(),
                bank_account_view_repository.clone(),
            ),
        );
    }

    axum_router = axum_router
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
        .layer(cors_layer.clone())
        .layer(CookieManagerLayer::new())
        .route("/health", get(|| async move { "HEALTHY" }));

    let web_server_config = WebServerConfiguration::from_env();
    let auth_application_service = AuthServiceImpl::new(Arc::new(user_repository));
    let auth_grpc_service =
        auth_grpc_service::GRpcAuthService::new(Box::new(auth_application_service));
    let bank_account_application_service =
        BankAccountServiceImpl::new(bank_account_view_repository.clone());
    let bank_account_service =
        GRpcBankAccountService::new(Box::new(bank_account_application_service));
    let bank_account_service_server = BankAccountServiceServer::new(bank_account_service);
    let grpc_web_bank_account_service = tonic_web::enable(bank_account_service_server);
    let tonic_greeter_service = tonic_web::enable(GreeterServer::new(MyGreeter::default()));
    let auth_server = tonic_web::enable(AuthenticationServer::new(auth_grpc_service));
    let grpc_router = Router::new()
        .nest_tonic(tonic_greeter_service)
        .nest_tonic(grpc_web_bank_account_service)
        .nest_tonic(auth_server)
        .layer(cors_layer);
    let multiplexed_service = RestGrpcService::new(axum_router, grpc_router);
    Ok(Server::bind(&web_server_config.get_address())
        .serve(multiplexed_service.into_make_service())
        .await?)
}
