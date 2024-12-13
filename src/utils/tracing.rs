use std::time::Duration;
use anyhow::Error;
use opentelemetry::global;
use opentelemetry_otlp::{WithExportConfig};
use opentelemetry_sdk::trace::Sampler;

pub fn trace() -> Result<(), Error> {
    /*let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint("http://localhost:4317")
                .build()?,
            opentelemetry_sdk::runtime::Tokio,
        )
        .build();

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4318/v1/metrics")
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_interval(std::time::Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .build();*/
    
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .with_timeout(Duration::from_secs(3))
        .build()?;
    
    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_sampler(Sampler::AlwaysOn)
        .build();
    global::set_tracer_provider(tracer_provider);
    Ok(())
}