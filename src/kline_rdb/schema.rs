use crate::impl_schema_candle_key_codec;
use crate::kline_rdb::candle_key::KLineCandleKey;
use bincode::{Decode, Encode};
use rksdb_infra::schemadb::ColumnFamilyName;
use rksdb_infra::{define_pub_schema, impl_schema_value_bin_codec};
use serde::{Deserialize, Serialize};

pub const KLINE_MINUTE1_CFN: ColumnFamilyName = "kline_min1_eds";
pub const KLINE_MINUTE5_CFN: ColumnFamilyName = "kline_min5_eds";
pub const KLINE_MINUTE15_CFN: ColumnFamilyName = "kline_min15_eds";
pub const KLINE_MINUTE30_CFN: ColumnFamilyName = "kline_min30_eds";
// /// 1 hour
// pub const KLINE_HOUR1_CFN: ColumnFamilyName = "kline_hour1_eds";
// /// 4 hour
// pub const KLINE_HOUR4_CFN: ColumnFamilyName = "kline_hour4_eds";
// pub const KLINE_DAILY_CFN: ColumnFamilyName = "kline_daily_eds";
// pub const KLINE_WEEKLY_CFN: ColumnFamilyName = "kline_weekly_eds";
// pub const KLINE_MONTHLY_CFN: ColumnFamilyName = "kline_monthly_eds";
// /// year
// pub const KLINE_YEARLY_CFN: ColumnFamilyName = "kline_yearly_eds";

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

// // 1h Candle
// define_pub_schema!(KLineHour1Schema, KLineCandleKey, Candle, KLINE_HOUR1_CFN);
// impl_schema_value_bin_codec!(KLineHour1Schema, Candle);
// impl_schema_candle_key_codec!(KLineHour1Schema);
//
// // 4h Candle
// define_pub_schema!(KLineHour4Schema, KLineCandleKey, Candle, KLINE_HOUR4_CFN);
// impl_schema_value_bin_codec!(KLineHour4Schema, Candle);
// impl_schema_candle_key_codec!(KLineHour4Schema);
// // impl_schema_kline_prefix_key_codec!(KLineHour4Schema);
//
// // daily Candle
// define_pub_schema!(KLineDailySchema, KLineCandleKey, Candle, KLINE_DAILY_CFN);
// impl_schema_value_bin_codec!(KLineDailySchema, Candle);
// impl_schema_candle_key_codec!(KLineDailySchema);
// // impl_schema_kline_prefix_key_codec!(KLineDailySchema);
//
// // weekly Candle
// define_pub_schema!(KLineWeeklySchema, KLineCandleKey, Candle, KLINE_WEEKLY_CFN);
// impl_schema_value_bin_codec!(KLineWeeklySchema, Candle);
// impl_schema_candle_key_codec!(KLineWeeklySchema);
// // impl_schema_kline_prefix_key_codec!(KLineWeeklySchema);
//
// // Monthly Candle
// define_pub_schema!(
//     KLineMonthlySchema,
//     KLineCandleKey,
//     Candle,
//     KLINE_MONTHLY_CFN
// );
// impl_schema_value_bin_codec!(KLineMonthlySchema, Candle);
// impl_schema_candle_key_codec!(KLineMonthlySchema);
// // impl_schema_kline_prefix_key_codec!(KLineMonthlySchema);
//
// // yearly Candle
// define_pub_schema!(KLineYearlySchema, KLineCandleKey, Candle, KLINE_YEARLY_CFN);
// impl_schema_value_bin_codec!(KLineYearlySchema, Candle);
// impl_schema_candle_key_codec!(KLineYearlySchema);
// // impl_schema_kline_prefix_key_codec!(KLineYearlySchema);
