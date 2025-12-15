use crate::error::ApiErr;
use base_infra::result::{AppError, AppResult};
use base_infra::{err, map_err};
use serde::{Deserialize, Deserializer, Serialize, de::Error as DeError};
use std::str::FromStr;

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
    /// 1 Hour
    Hourly,
    /// 4 Hour
    Hour4,
}
impl FromStr for Resolution {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1m" => Ok(Self::Min1),
            "5m" => Ok(Self::Min5),
            "15m" => Ok(Self::Min15),
            "30m" => Ok(Self::Min30),
            "1H" => Ok(Self::Hourly),
            "4H" => Ok(Self::Hour4),
            _ => {
                let s = format!("{}, can only be(1m,5m,15m,30m,1H,4H,1D,1W,1M)", s);
                err!(&ApiErr::InvalidResolution, s)
            }
        }
    }
}

/// KLine query parameters
#[derive(Clone, Debug, Default, Deserialize)]
pub struct KLineQueryParams {
    /// KLine resolution(e.g. 1m,5m,15m,30m,1H,4H,1D,1W,1M)
    #[serde(default = "default_resolution", deserialize_with = "deser_resolution")]
    pub resolution: Resolution,
    /// Token0  address
    pub token0: String,
    /// Token1  address
    pub token1: String,
    /// unix timestamp (UTC) of leftmost required bar
    pub from: Option<u64>,
    /// unix timestamp (UTC) of rightmost required bar (not inclusive)
    pub to: Option<u64>,
    /// number of bars (higher priority than from) starting with to. If countback is set, from should be ignored.
    pub countback: Option<u64>,
}

// impl From<AppError> for dyn DeError {
//     fn from(value: AppError) -> Box<Self> {
//         todo!()
//     }
// }

// pub fn deser_resolution<'de, D>(s: D) -> AppResult<Resolution>
// where
//     D: Deserializer<'de>,
// {
//     let raw = String::deserialize(s).map_err(map_err!(&ApiErr::DeserStr))?;
//     Resolution::from_str(&raw)
// }

pub fn deser_resolution<'de, D>(input: D) -> Result<Resolution, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(input)?;
    Resolution::from_str(&s)
        .map_err(|_| DeError::custom(format!("{s}(input), only(1m,5m,15m,30m,1H,4H,1D,1W,1M)")))
}

fn default_resolution() -> Resolution {
    Resolution::Min1
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        println!("compile successful");
    }
}
