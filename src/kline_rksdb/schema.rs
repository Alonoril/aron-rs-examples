use crate::impl_schema_candle_key_codec;
use crate::kline_rksdb::candle_key::KLineCandleKey;
use bincode::{Decode, Encode};
use rksdb_infra::schemadb::ColumnFamilyName;
use rksdb_infra::{define_pub_schema, impl_schema_value_bin_codec};
use serde::{Deserialize, Serialize};

pub const KLINE_MINUTE1_CFN: ColumnFamilyName = "kline_min1_eds";
pub const KLINE_MINUTE5_CFN: ColumnFamilyName = "kline_min5_eds";
pub const KLINE_MINUTE15_CFN: ColumnFamilyName = "kline_min15_eds";
pub const KLINE_MINUTE30_CFN: ColumnFamilyName = "kline_min30_eds";

#[derive(Clone, Debug, Default, PartialEq, Encode, Decode, Serialize, Deserialize)]
pub struct Candle {
    // pub ts: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub txn_cnt: i64,
    pub first_ts: i64,
    pub last_ts: i64,
    pub initialized: bool,
}

// 1m Candle
define_pub_schema!(KLineMin1Schema, KLineCandleKey, Candle, KLINE_MINUTE1_CFN);
impl_schema_value_bin_codec!(KLineMin1Schema, Candle);
impl_schema_candle_key_codec!(KLineMin1Schema);

// 5m Candle
define_pub_schema!(KLineMin5Schema, KLineCandleKey, Candle, KLINE_MINUTE5_CFN);
impl_schema_value_bin_codec!(KLineMin5Schema, Candle);
impl_schema_candle_key_codec!(KLineMin5Schema);

// 15m Candle
define_pub_schema!(KLineMin15Schema, KLineCandleKey, Candle, KLINE_MINUTE15_CFN);
impl_schema_value_bin_codec!(KLineMin15Schema, Candle);
impl_schema_candle_key_codec!(KLineMin15Schema);

// 30m Candle
define_pub_schema!(KLineMin30Schema, KLineCandleKey, Candle, KLINE_MINUTE30_CFN);
impl_schema_value_bin_codec!(KLineMin30Schema, Candle);
impl_schema_candle_key_codec!(KLineMin30Schema);
