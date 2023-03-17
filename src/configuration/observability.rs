use tracing::instrument;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[instrument]
pub async fn configure_tracing() -> std::result::Result<(), crate::error::Error> {
    // Configure the OpenTelemetry tracer
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // TODO: Use the OTEL collector instead of going directly to Jaeger

    // Get the service name from the environment
    let tracing_service_name =
        dotenvy::var("TRACING_SERVICE_NAME").unwrap_or_else(|_| "veloxide-service".to_string());

    // Create a new Jaeger tracer
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(tracing_service_name.clone())
        .install_simple()
        // .install_batch(opentelemetry::runtime::Tokio) // Uncomment me to use batch span processor
        .expect("Expected Jaeger tracer to install successfully, regardless of whether Jaeger is running or not");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Create a new formatting layer to print bunyan formatted logs to stdout, pipe into bunyan to view
    let formatting_layer = BunyanFormattingLayer::new(tracing_service_name, std::io::stdout);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = tracing_subscriber::Registry::default()
        .with(formatting_layer)
        .with(telemetry);

    Ok(tracing::subscriber::set_global_default(subscriber)?)
}
