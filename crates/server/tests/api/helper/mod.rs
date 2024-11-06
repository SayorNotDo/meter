use once_cell::sync::Lazy;
use tracing::Subscriber;
use server::configure;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::{SubscriberExt, Context}, EnvFilter, Registry, Layer};
use tracing_subscriber::registry::LookupSpan;

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
            .with(BunyanFormattingLayer::new(
                "test-server".into(),
                std::io::stdout,
            ))
            .with(TestCaseLayer),
    )
    .unwrap()
});

struct TestCaseLayer;

impl<S> Layer<S> for TestCaseLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &tracing::span::Attributes, id: &tracing::span::Id, ctx: Context<'_, S>) {
        // Get the current test case name from the test framework
        let test_case_name = std::env::var("TEST_CASE_NAME").unwrap_or_else(|_| "unknown".to_string());

        // Add a custom field to the span with the test case name
        let span = ctx.span(id).unwrap();
    }
}