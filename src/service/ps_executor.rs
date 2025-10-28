use crate::types::ps_types::PrivateJob;
use base_infra::result::AppResult;
use base_infra::runtimes::Tokio;
// use rand::prelude::SliceRandom;
// use rand::thread_rng;
use tracing::{Instrument, error, info, instrument, warn};

#[derive(Clone)]
pub struct PrivateSaleExecutor {
    // other repositories and services
}

impl PrivateSaleExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn spawn(&self, job: PrivateJob) -> AppResult<()> {
        let job_id = job.job_id;
        info!("Starting private sale task，job_id: {job_id}");
        let this = self.clone();
        Tokio.spawn(
            async move {
                if let Err(err) = this.execute(job).await {
                    error!("job_id[{job_id}]: Exec private sale purchases error: {err:?}");
                }
            }
            .in_current_span(),
        );
        Ok(())
    }

    #[instrument(skip_all, "ps", fields(id=job.job_id))]
    async fn execute(&self, job: PrivateJob) -> AppResult<()> {
        // 第一次在这里使用，报错：
        // error: future cannot be sent between threads safely
        //   --> src/service/ps_executor.rs:22:15
        //    |
        // 22 |         Tokio.spawn(
        //    |               ^^^^^ future created by async block is not `Send`
        // let mut rng = thread_rng();
        // let wallets: Vec<_> = job
        //     .wallets
        //     .choose_multiple(&mut rng, job.purchase_size)
        //     .cloned()
        //     .collect();
        // for wallet in wallets {

        for wallet in &job.wallets {
            self.do_purchase(wallet).await?;
        }

        Ok(())
    }

    #[instrument(skip_all, "buy", fields(wallet = wallet))]
    async fn do_purchase(&self, wallet: &str) -> AppResult<()> {
        // 模拟购买
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        warn!("do purchase private sale");
        Ok(())
    }
}
