pub mod candle_key;
pub mod kline_iter;
pub mod kline_storage;
pub mod schema;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resolution {
    /// 1 Minute
    #[default]
    Min1 = 1,
    /// 5 Minute
    Min5,
    /// 15 Minute
    Min15,
    /// 30 Minute
    Min30,
}
impl Resolution {
    pub fn values() -> [Resolution; 1] {
        [
            Resolution::Min1,
            // Resolution::Min5,
            // Resolution::Min15,
            // Resolution::Min30,
        ]
    }
}

#[derive(Clone, Debug, Default)]
pub struct KLineHisQuery {
    pub res: Resolution,
    pub pool_id: i32,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub limit: Option<u64>,
}

impl KLineHisQuery {
    pub fn new(res: Resolution, pool_id: i32) -> Self {
        Self {
            pool_id,
            res,
            ..Default::default()
        }
    }
}
