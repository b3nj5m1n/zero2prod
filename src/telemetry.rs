use anyhow::Result;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

// Creates a new file with filename timestamp_suffix.log in XDG_CACHE_HOME
pub fn get_log_file<'a>(suffix: String) -> Result<std::fs::File> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("zero2prod")?;
    let timestamp = chrono::Utc::now().format("%Y_%m_%d__%H_%M_%S");
    let path = xdg_dirs.place_cache_file(format!("{timestamp}__{suffix}.log"))?;
    Ok(std::fs::File::create(path)?)
}

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let layer_bunyan = BunyanFormattingLayer::new(name, sink);
    let layer_stdout = tracing_subscriber::fmt::layer();
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(layer_bunyan)
        .with(layer_stdout)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
}
