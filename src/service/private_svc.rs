use crate::service::ps_executor::PrivateSaleExecutor;
use crate::types::ps_types::PrivateJob;
use base_infra::result::AppResult;
use std::sync::Arc;
use rand::prelude::{SliceRandom, StdRng};
use rand::{thread_rng, SeedableRng};
use tracing::info;

pub type DynPsService = Arc<dyn PsServiceTrait + Send + Sync>;

#[async_trait::async_trait]
pub trait PsServiceTrait {
    async fn start_private_sale(&self, job: PrivateJob) -> AppResult<()>;
}

#[derive(Clone)]
pub struct PsService {
    pub ps_executor: PrivateSaleExecutor,
}

impl PsService {
    pub fn new() -> Self {
        Self {
            ps_executor: PrivateSaleExecutor::new(),
        }
    }
}

#[async_trait::async_trait]
impl PsServiceTrait for PsService {
    async fn start_private_sale(&self, job: PrivateJob) -> AppResult<()> {
        // 模拟从数据库获取地址列表
        let wallets = vec![
            "BucFEyoW27MHx7qzPxduv7gmFGJ4MSay8ejiUd2hQYnc".to_string(),
            "5k686c423sSDYWCYH8WCf28DYhjB18z5uNq2wmf1J2Ei".to_string(),
            "FUH7QsNDZmsFLEXrycR9pXQer2BSgGoUQ84b4RVJkWND".to_string(),
            "6U4J1isVmrua5YRVobNQRCybRHPPgh5sobTURaPJtL4Q".to_string(),
            "GkK1EvrD8HPnUz1TFCak1ss2i5r7qn8YpRnPZoDf9xyd".to_string(),
            "B28ezxibEf7eNAGw7A3nFWe7yXhpuvr9S4disfgWGEq6".to_string(),
        ];
        info!("Starting private sale with {} wallets", wallets.len());

        // 将随机前置到这里
        // let mut rng = thread_rng();
        // 并且使用使用 rand::rngs::StdRng包装一下
        let mut rng = StdRng::from_rng(thread_rng()).unwrap();
        let wallets: Vec<String> = wallets
            .choose_multiple(&mut rng, job.purchase_size)
            .cloned()
            .collect();
        info!("new wallets: {:?}", wallets);

        let job = job.with_wallets(wallets);
        // Start the job
        self.ps_executor.spawn(job).await?;

        Ok(())
    }
}
