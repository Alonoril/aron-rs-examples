use crate::kline_rksdb::KLineHisQuery;
use crate::kline_rksdb::candle_key::KLineCandleKey;
use crate::kline_rksdb::kline_iter::KLineIter;
use crate::kline_rksdb::schema::{
    Candle, KLINE_MINUTE1_CFN, KLINE_MINUTE5_CFN, KLINE_MINUTE15_CFN, KLINE_MINUTE30_CFN,
    KLineMin1Schema,
};
use base_infra::result::AppResult;
use chain_types::endless::AccountAddress;
use chrono::Utc;
use rksdb_cfg::RksDbDirPaths;
use rksdb_infra::schemadb::{ColumnFamilyName, RksDB};
use rksdb_infra::{DEFAULT_COLUMN_FAMILY_NAME, OpenRocksDB};
use std::path::PathBuf;
use std::sync::Arc;

/// KLine rocksdb
pub struct KlineRdb {
    pub rdb: Arc<RksDB>,
}

impl KlineRdb {
    pub fn new_self(rdb: Arc<RksDB>) -> Self {
        KlineRdb { rdb }
    }

    pub fn list_kline_history(
        &self,
        query: &KLineHisQuery,
        token: AccountAddress,
    ) -> AppResult<Vec<(KLineCandleKey, Candle)>> {
        let limit = query.limit.unwrap_or(500) as usize;
        let start = query.start.map(|e| e.and_utc().timestamp() as u64);

        let mut list = vec![];
        let mut iter = self.iter_candles(query, token)?;
        while let Some(res) = iter.next() {
            let (key, val) = res?;
            if let Some(start) = start {
                if key.2 < start {
                    break;
                }
            } else {
                if list.len() >= limit {
                    break;
                }
            }

            list.push((key, val));
        }
        Ok(list)
    }

    fn iter_candles(
        &self,
        query: &KLineHisQuery,
        token: AccountAddress,
    ) -> AppResult<KLineIter<KLineMin1Schema>> {
        let end = query.end.unwrap_or(Utc::now().naive_utc());
        let end = end.and_utc().timestamp();

        let prefix = KLineCandleKey::new(query.pool_id, token, end);
        // Normal order query
        let mut iter = self.rdb.iter::<KLineMin1Schema>()?;
        iter.seek_for_prev(&prefix)?;

        let pid = query.pool_id as u32;
        Ok(KLineIter::new(iter, pid, token, end as u64))
    }
}

impl OpenRocksDB for KlineRdb {
    fn new_inner(db: RksDB) -> AppResult<Self>
    where
        Self: Sized,
    {
        Ok(Self { rdb: Arc::new(db) })
    }

    fn get_db_column_families() -> Vec<ColumnFamilyName> {
        vec![
            /* empty cf */ DEFAULT_COLUMN_FAMILY_NAME,
            KLINE_MINUTE1_CFN,
            KLINE_MINUTE5_CFN,
            KLINE_MINUTE15_CFN,
            KLINE_MINUTE30_CFN,
        ]
    }

    fn get_db_path(db_paths: RksDbDirPaths) -> PathBuf {
        db_paths.rdb_root_path().join("kline_db")
    }
}
