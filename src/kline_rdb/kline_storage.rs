use crate::kline_rdb::candle_key::KLineCandleKey;
use crate::kline_rdb::kline_iter::KLineIter;
use crate::kline_rdb::schema::{
    Candle, KLineMin1Schema, KLineMin5Schema, KLineMin15Schema, KLineMin30Schema,
};
use crate::kline_rdb::{KLineHisQuery, Resolution};
use base_infra::result::AppResult;
use chain_types::endless::AccountAddress;
use chrono::Utc;
use rksdb_infra::schemadb::schema::KeyCodec;
use rksdb_infra::schemadb::{RksDB, Schema};
use std::sync::Arc;

/// KLine rocksdb
pub struct KlineRdb {
    pub rdb: Arc<RksDB>,
}

impl KlineRdb {
    pub fn new(rdb: Arc<RksDB>) -> Self {
        KlineRdb { rdb }
    }

    fn list_kline_history(
        &self,
        query: &KLineHisQuery,
        token: AccountAddress,
    ) -> AppResult<Vec<(KLineCandleKey, Candle)>> {
        match query.res {
            Resolution::Min1 => self.collect_kline::<KLineMin1Schema>(query, token),
            Resolution::Min5 => self.collect_kline::<KLineMin5Schema>(query, token),
            Resolution::Min15 => self.collect_kline::<KLineMin15Schema>(query, token),
            Resolution::Min30 => self.collect_kline::<KLineMin30Schema>(query, token),
        }
    }

    fn collect_kline<S>(
        &self,
        query: &KLineHisQuery,
        token: AccountAddress,
    ) -> AppResult<Vec<(KLineCandleKey, Candle)>>
    where
        S: Schema<Key = KLineCandleKey, Value = Candle>,
        KLineCandleKey: KeyCodec<S>,
    {
        let limit = query.limit.unwrap_or(500) as usize;
        let start = query.start.map(|e| e.and_utc().timestamp() as u64);

        let mut iter: KLineIter<S> = self.iter_candles(query, token)?;
        let mut list = vec![];

        while let Some(res) = iter.next() {
            let (key, val) = res?;

            // 示例过滤逻辑（按你原注释改）
            if let Some(start) = start {
                if key.2 < start {
                    break;
                }
            } else if list.len() >= limit {
                break;
            }

            list.push((key, val));
        }
        Ok(list)
    }

    fn iter_candles<S>(
        &self,
        query: &KLineHisQuery,
        token: AccountAddress,
    ) -> AppResult<KLineIter<'_, S>>
    where
        S: Schema<Key = KLineCandleKey, Value = Candle>,
        KLineCandleKey: KeyCodec<S>,
    {
        let end = query.end.unwrap_or(Utc::now().naive_utc());
        let end_ts = end.and_utc().timestamp();

        let prefix = KLineCandleKey::new(query.pool_id, token, end_ts);
        let mut iter = self.rdb.rev_iter::<S>()?;
        iter.seek_for_prev(&prefix)?;
        // let mut iter = self.rdb.iter::<KLineMin1Schema>()?;
        // iter.seek_to_first();

        let pid = query.pool_id as u32;
        Ok(KLineIter::new(iter, pid, token, end_ts as u64))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
