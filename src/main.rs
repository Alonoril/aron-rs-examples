use aron_rs_examples::types::ps_types::PrivateJob;
use aron_rs_examples::{Services, init_tracing};
use base_infra::result::AppResult;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

mod service;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing();
    let svc = Services::new();

    start_ps(&svc).await?;

    let term = Arc::new(AtomicBool::new(false));
    while !term.load(Ordering::Acquire) {
        thread::park();
    }
    Ok(())
}

async fn start_ps(svc: &Services) -> AppResult<()> {
    let job = PrivateJob {
        job_id: 1,
        interval_min: 1,
        interval_max: 5,
        wallets: vec![],
        purchase_size: 3,
    };

    svc.ps_svc.start_private_sale(job).await?;

    Ok(())
}
