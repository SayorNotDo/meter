use once_cell::sync::Lazy;
use server::configure;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, fmt::Layer, layer::SubscriberExt, EnvFilter, Registry,
};

pub mod api;
pub mod assert;
pub mod project;
pub mod result;
pub mod user;

pub(crate) static INIT_SUBSCRIBER: Lazy<()> = Lazy::new(|| {
    configure::tracing::init_subscriber(
        Registry::default()
            .with(EnvFilter::new("INFO"))
            .with(JsonStorageLayer)
            .with(
                Layer::new()
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_span_events(FmtSpan::FULL)
                    .without_time(),
            ),
    )
    .unwrap()
});
