use once_cell::sync::Lazy;
use server::configure;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub mod api;
pub mod assert;
pub mod result;
pub mod user;

pub(crate) static INIT_SUBCRIBER: Lazy<()> = Lazy::new(|| {
    configure::tracing::init_subscriber(
        Registry::default()
            .with(EnvFilter::new("INFO"))
            .with(JsonStorageLayer)
            .with(BunyanFormattingLayer::new(
                "test-app".into(),
                std::io::stdout,
            )),
    )
    .unwrap()
});
