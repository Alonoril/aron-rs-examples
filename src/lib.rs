use crate::service::private_svc::{DynPsService, PsService};
use base_infra::WorkerGuard;
use std::sync::Arc;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

pub mod errors;
pub mod service;
pub mod types;

pub struct Services {
    pub ps_svc: DynPsService,
}

impl Services {
    pub fn new() -> Self {
        let ps_svc = Arc::new(PsService::new()) as DynPsService;
        Self { ps_svc }
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
