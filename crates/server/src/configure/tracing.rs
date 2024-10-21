use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
// use opentelemetry_otlp::WithExportConfig;
use crate::errors::AppResult;
// use std::collections::HashMap;
// use std::time::Duration;
use tracing::{subscriber, Subscriber};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_opentelemetry;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

fn create_subscriber<W>(
    name: &str,
    env_filter: EnvFilter,
    writer: W,
) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    /* Opentelemetry */
    let provider = TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    // let mut headers = HashMap::with_capacity(2);
    // headers.insert(
    //     "Authorization".into(),
    //     "Basic cm9vdEBleGFtcGxlLmNvbTp1c3JRQWdtMHVIaVRoUEtv".into(),
    // );
    // headers.insert("stream-name".into(), "default".into());

    // let exporter = opentelemetry_otlp::new_exporter()
    //     .http()
    //     .with_headers(headers)
    //     .with_endpoint("http://192.168.50.134:5080/api/default/default/_json".to_string())
    //     .with_timeout(Duration::from_secs(3));

    // let provider = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(exporter)
    //     .install_batch(opentelemetry_sdk::runtime::Tokio)
    //     .expect("initialize tracing provider failure...");

    let tracer = provider.tracer("tracing_server");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name.into(), std::io::stdout))
        .with(BunyanFormattingLayer::new(name.into(), writer))
}

pub fn init_subscriber<S>(subscriber: S) -> anyhow::Result<()>
where
    S: Subscriber + Send + Sync + 'static,
{
    LogTracer::init()?;
    subscriber::set_global_default(subscriber)?;
    Ok(())
}

pub fn init() -> AppResult<WorkerGuard> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "server.log");
    let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
    init_subscriber(create_subscriber(
        "server",
        EnvFilter::from_default_env(),
        file_appender,
    ))?;
    Ok(file_appender_guard)
}
