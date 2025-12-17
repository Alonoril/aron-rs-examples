use aron_rs_examples::{Services, init_tracing};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing();
    let svc = Services::new();

    let term = Arc::new(AtomicBool::new(false));
    while !term.load(Ordering::Acquire) {
        thread::park();
    }
    Ok(())
}
