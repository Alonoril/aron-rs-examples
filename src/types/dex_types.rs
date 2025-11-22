use serde::{Deserialize, Serialize, de::Error as DeError};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub enum DexId {
    #[default]
    Unknown,
    UniV2,
    UniV3,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "deserialize_token_decimals")]
    pub decimals: i32,
}

#[derive(Debug, Deserialize)]
pub struct PoolInfo {
    pub id: String,
    #[serde(
        rename = "volumeUSD",
        default,
        deserialize_with = "deserialize_volume_usd"
    )]
    pub volume_usd: Option<f64>,
    pub token0: Token,
    pub token1: Token,
    #[serde(skip_deserializing)]
    pub dex_id: DexId,
}

impl PoolInfo {
    pub fn with_dex_id(self, dex_id: DexId) -> Self {
        Self { dex_id, ..self }
    }

    // pub fn with_dex_id(&mut self, dex_id: DexId) {
    //     self.dex_id = dex_id;
    // }
}

#[derive(Deserialize, Debug)]
pub struct PoolsResponse {
    pub pools: Vec<PoolInfo>,
}

fn deserialize_volume_usd<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawVolume {
        Number(f64),
        String(String),
        Null,
    }

    match RawVolume::deserialize(deserializer)? {
        RawVolume::Number(n) => Ok(Some(n)),
        RawVolume::String(s) => {
            let parsed = s
                .parse::<f64>()
                .map_err(|_| DeError::custom("invalid volumeUSD"))?;
            Ok(Some(parsed))
        }
        RawVolume::Null => Ok(None),
    }
}

fn deserialize_token_decimals<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawDecimals {
        Int(i32),
        Int64(i64),
        String(String),
    }

    match RawDecimals::deserialize(deserializer)? {
        RawDecimals::Int(v) => Ok(v),
        RawDecimals::Int64(v) => Ok(v as i32),
        RawDecimals::String(s) => s
            .parse::<i32>()
            .map_err(|_| DeError::custom("invalid token decimals")),
    }
}
