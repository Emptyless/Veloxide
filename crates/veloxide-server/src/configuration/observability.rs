use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Layer};

const OTEL_EXPORTER_OTLP_ENDPOINT_ENV_VAR: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
const OTEL_EXPORTER_OTLP_ENDPOINT_DEFAULT: &str = "http://localhost:4317";

const OBSERVABILITY_SERVICE_NAME_ENV_VAR: &str = "OBSERVABILITY_SERVICE_NAME";
const DEFAULT_SERVICE_NAME: &str = "veloxide-server";

#[tracing::instrument]
pub async fn configure_tracing() -> std::result::Result<(), crate::error::Error> {
    let otel_exporter_endpoint =
        dotenvy::var(OTEL_EXPORTER_OTLP_ENDPOINT_ENV_VAR).unwrap_or_else(|_| {
            tracing::warn!(
                "{} Env var not set, using default",
                OTEL_EXPORTER_OTLP_ENDPOINT_ENV_VAR
            );
            OTEL_EXPORTER_OTLP_ENDPOINT_DEFAULT.to_string()
        });

    let tracing_service_name = dotenvy::var(OBSERVABILITY_SERVICE_NAME_ENV_VAR)
        .unwrap_or_else(|_| DEFAULT_SERVICE_NAME.to_string());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otel_exporter_endpoint),
        )
        .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
            opentelemetry::sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                tracing_service_name,
            )]),
        ))
        .install_batch(opentelemetry::runtime::Tokio)?;

    // Create a tracing layer with the configured tracer
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let filter = tracing_subscriber::EnvFilter::from_default_env();

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = tracing_subscriber::Registry::default()
        .with(telemetry_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(filter),
        );

    Ok(tracing::subscriber::set_global_default(subscriber)?)
}
