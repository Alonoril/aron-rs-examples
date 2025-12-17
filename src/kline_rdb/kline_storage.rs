use rksdb_infra::schemadb::RksDB;
use std::sync::Arc;
use base_infra::result::AppResult;
use chain_types::endless::AccountAddress;
use crate::kline_rdb::candle_key::KLineCandleKey;
use crate::kline_rdb::{KLineHisQuery, Resolution};
use crate::kline_rdb::schema::{Candle, KLineMin15Schema, KLineMin1Schema, KLineMin30Schema, KLineMin5Schema};

/// KLine rocksdb
pub struct KlineRdb {
    pub rdb: Arc<RksDB>,
}

impl KlineRdb {
    pub fn new(rdb: Arc<RksDB>) -> Self {
        KlineRdb { rdb }
    }

    // fn list_kline_history(
    //     &self,
    //     query: &KLineHisQuery,
    //     token: AccountAddress,
    // ) -> AppResult<Vec<(KLineCandleKey, Candle)>> {
    //     match query.res {
    //         Resolution::Min1 => self.collect_kline::<KLineMin1Schema>(query, token),
    //         Resolution::Min5 => self.collect_kline::<KLineMin5Schema>(query, token),
    //         Resolution::Min15 => self.collect_kline::<KLineMin15Schema>(query, token),
    //         Resolution::Min30 => self.collect_kline::<KLineMin30Schema>(query, token),
    //     }
    // }

}
