use crate::kline_rksdb::candle_key::KLineCandleKey;
use crate::kline_rksdb::schema::Candle;
use base_infra::result::AppResult;
use chain_types::endless::AccountAddress;
use rksdb_infra::schemadb::Schema;
use rksdb_infra::schemadb::iterator::SchemaIterator;

pub struct KLineIter<'a, S: Schema> {
    inner: SchemaIterator<'a, S>,
    pub pid: u32,
    pub token: AccountAddress,
    pub end_ts: u64,
}
impl<'a, S> KLineIter<'a, S>
where
    S: Schema<Key = KLineCandleKey, Value = Candle>,
{
    pub fn new(inner: SchemaIterator<'a, S>, pid: u32, token: AccountAddress, end_ts: u64) -> Self {
        Self {
            inner,
            pid,
            token,
            end_ts,
        }
    }
    fn next_impl(&mut self) -> AppResult<Option<(KLineCandleKey, Candle)>> {
        match self.inner.next().transpose()? {
            Some((key, val)) => {
                if key.0 != self.pid || key.1 != self.token {
                    return Ok(None);
                }
                Ok(Some((key, val)))
            }
            None => Ok(None),
        }
    }
}
impl<'a, S: Schema> Iterator for KLineIter<'a, S>
where
    S: Schema<Key = KLineCandleKey, Value = Candle>,
{
    type Item = AppResult<(KLineCandleKey, Candle)>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl().transpose()
    }
}
