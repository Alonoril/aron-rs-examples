use base_infra::WorkerGuard;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

pub mod error;
mod exams;
mod kline_api;
pub mod kline_rdb;
pub mod service;
pub mod types;

pub struct Services {}

impl Services {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn init_tracing() -> WorkerGuard {
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    let layer = Layer::new()
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_ansi(true)
        .with_writer(non_blocking);

    let layered = registry()
        // .with(max_level)
        .with(layer);

    layered.init();
    guard
}
