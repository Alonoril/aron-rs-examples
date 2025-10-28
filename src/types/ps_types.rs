use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateJob {
    pub job_id: i32,
    pub interval_min: i32,
    pub interval_max: i32,
    #[serde(skip)]
    pub wallets: Vec<String>,
    pub purchase_size: usize,
}

impl PrivateJob {
    pub fn with_wallets(self, wallets: Vec<String>) -> Self {
        Self { wallets, ..self }
    }

    pub fn random_interval(&self) -> u64 {
        let (min, max) = (self.interval_min, self.interval_max);
        if min >= max {
            return min as u64;
        }

        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max) as u64
    }
}
