use base_infra::result::{AppResult, any_err_ext};
use base_infra::{err, map_err};
use chain_types::endless::AccountAddress;
use chain_types::endless::eds_addr_ext::ToEdsAddr;

const EDS_ADDR_LEN: usize = AccountAddress::LENGTH;

/// (pool_id, token, bucket(timestamp))
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KLineCandleKey(pub u32, pub AccountAddress, pub u64);

impl KLineCandleKey {
    pub fn new(pool_id: i32, token: AccountAddress, timestamp: i64) -> Self {
        KLineCandleKey(pool_id as u32, token, timestamp as u64)
    }
}

/// (pool_id, token)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KLinePrefixKey(pub u32, pub AccountAddress);
impl KLinePrefixKey {
    pub fn new(pool_id: i32, token: AccountAddress) -> Self {
        KLinePrefixKey(pool_id as u32, token)
    }
}

#[macro_export]
macro_rules! impl_schema_candle_key_codec {
	($schema_type:ty) => {
		impl ::rksdb_infra::schemadb::schema::KeyCodec<$schema_type>
			for $crate::kline_rksdb::candle_key::KLineCandleKey
		{
			fn encode_key(&self) -> base_infra::result::AppResult<Vec<u8>> {
				$crate::kline_rksdb::candle_key::encode_key(self)
			}

			fn decode_key(data: &[u8]) -> base_infra::result::AppResult<Self> {
				$crate::kline_rksdb::candle_key::decode_key(data)
			}
		}
	};
}

#[macro_export]
macro_rules! impl_schema_kline_prefix_key_codec {
	($schema_type:ty) => {
		impl ::rksdb_infra::schemadb::schema::SeekKeyCodec<$schema_type>
			for $crate::kline_rksdb::candle_key::KLinePrefixKey
		{
			fn encode_seek_key(&self) -> base_infra::result::AppResult<Vec<u8>> {
				$crate::kline_rksdb::candle_key::encode_prefix(self)
			}
		}
	};
}

base_infra::gen_impl_code_enum! {
	KeyErr {
		TokenLen = ("RKY001", "token length mismatch:"),
		KeyLen = ("RKY002", "invalid key length:"),
		InvalidUtf8 = ("RKY003", "token is not valid utf-8"),
		DecKey = ("RKY004", "decode key error"),
	}
}

pub fn encode_key(key: &KLineCandleKey) -> AppResult<Vec<u8>> {
    let KLineCandleKey(pool_id, token, bucket_ts) = key;
    let tb = token.into_bytes();

    let mut out = Vec::with_capacity(4 + EDS_ADDR_LEN + 8);
    out.extend_from_slice(&pool_id.to_be_bytes()); // u32 big-endian
    out.extend_from_slice(&tb); // 固定长度 token bytes
    out.extend_from_slice(&bucket_ts.to_be_bytes()); // u64 big-endian
    Ok(out)
}

pub fn decode_key(bytes: &[u8]) -> AppResult<KLineCandleKey> {
    let expected = 4 + EDS_ADDR_LEN + 8;
    if bytes.len() != expected {
        let msg = format!("expected {expected}, got {}", bytes.len());
        return err!(&KeyErr::KeyLen, msg);
    }

    let b_pid: [u8; 4] = bytes[0..4]
        .try_into()
        .map_err(any_err_ext(&KeyErr::DecKey, "invalid pool_id bytes len"))?;
    let pool_id = u32::from_be_bytes(b_pid);
    let token = (&bytes[4..4 + EDS_ADDR_LEN]).to_eds_addr()?;

    let ts_off = 4 + EDS_ADDR_LEN;
    let b_ts: [u8; 8] = bytes[ts_off..ts_off + 8]
        .try_into()
        .map_err(any_err_ext(&KeyErr::DecKey, "invalid bucket_ts bytes len"))?;
    let bucket_ts = u64::from_be_bytes(b_ts);

    Ok(KLineCandleKey(pool_id, token, bucket_ts))
}

/// (pool_id, token) 的前缀：用于 prefix seek 扫描某个 token 的最早/最新
pub fn encode_prefix(prefix: &KLinePrefixKey) -> AppResult<Vec<u8>> {
    let KLinePrefixKey(pool_id, token) = prefix;
    let tb = token.into_bytes();

    let mut out = Vec::with_capacity(4 + EDS_ADDR_LEN);
    out.extend_from_slice(&pool_id.to_be_bytes());
    out.extend_from_slice(&tb);
    Ok(out)
}

