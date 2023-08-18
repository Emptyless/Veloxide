use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};

use crate::domain::*;

pub struct SimpleLoggingQuery;

#[async_trait]
impl Query<BankAccount> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}

pub fn configure_logging() {
    tracing_log::LogTracer::builder()
        .ignore_crate("sqlx")
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("could not initialize log tracer");
}
