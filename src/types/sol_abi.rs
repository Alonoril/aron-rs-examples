use crate::error::{AcErr, any_err};
// use crate::{map_err_v1, map_err_v2};
use alloy_dyn_abi::DynSolType;
use alloy_primitives::Bytes;
use base_infra::result::AppResult;
use base_infra::{else_err, map_err};
use tracing::debug;

pub enum SolType {
    String,
    Bytes32,
}

pub trait SolTypeTrait {
    fn sol_type(&self) -> SolType;
}

impl SolTypeTrait for Bytes {
    fn sol_type(&self) -> SolType {
        if 32_usize.eq(&self.len()) {
            SolType::Bytes32
        } else {
            SolType::String
        }
    }
}

pub trait ToStrTrait {
    fn to_str(&self) -> AppResult<String>;
}
impl ToStrTrait for Bytes {
    fn to_str(&self) -> AppResult<String> {
        debug!("parse bytes: {:?}, len: {}", self, self.len());
        match (&self).sol_type() {
            SolType::String => sol_string_to_str(self),
            SolType::Bytes32 => Ok(sol_bytes32_to_str(self)),
        }
    }
}

// map_err_v1 Error：
//   --> src/types/sol_abi.rs:44:18
//    |
// 44 |         .map_err(map_err_v1!(&AcErr::IllSolTypeValue, "SolType(`string`)"))?;
//    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
// pub fn sol_string_to_str_v1(bs: &Bytes) -> AppResult<String> {
//     let ty: DynSolType = "string"
//         .parse()
//         .map_err(map_err_v1!(&AcErr::IllSolTypeValue, "SolType(`string`)"))?;
//     let res = ty
//         .abi_decode(&bs)
//         .map_err(map_err_v1!(&AcErr::EvmSolStrParseErr))?;
//
//
//     res.as_str().map(|s| s.to_string()).ok_or_else(else_err!(
//         &AcErr::IllSolTypeValue,
//         "Expected ABI decode result to be a String"
//     ))
// }

// error[E0562]: `impl Trait` is not allowed in closure parameters
//   --> src/errors.rs:29:15
//    |
// 29 |         |err: impl std::fmt::Debug + std::fmt::Display + Send + 'static| {
//    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//    |
//   ::: src/types/sol_abi.rs:64:18
//    |
// 64 |         .map_err(map_err_v2!(&AcErr::IllSolTypeValue, "SolType(`string`)"))?;
//    |                  --------------------------------------------------------- in this macro invocation
// pub fn sol_string_to_str(bs: &Bytes) -> AppResult<String> {
//     let ty: DynSolType = "string"
//         .parse()
//         .map_err(map_err_v2!(&AcErr::IllSolTypeValue, "SolType(`string`)"))?;
//     let res = ty
//         .abi_decode(&bs)
//         .map_err(map_err_v2!(&AcErr::EvmSolStrParseErr))?;
//
//     res.as_str().map(|s| s.to_string()).ok_or_else(else_err!(
//         &AcErr::IllSolTypeValue,
//         "Expected ABI decode result to be a String"
//     ))
// }

// pub fn sol_string_to_str(bs: &Bytes) -> AppResult<String> {
//     let ty: DynSolType = "string".parse().map_err(|err: alloy_dyn_abi::Error| {
//         map_err_v1!(&AcErr::IllSolTypeValue, "SolType(`string`)")(err)
//     })?;
//     let res = ty
//         .abi_decode(&bs)
//         .map_err(|err: alloy_dyn_abi::Error| map_err_v1!(&AcErr::IllSolTypeValue)(err))?;
//
//     res.as_str().map(|s| s.to_string()).ok_or_else(else_err!(
//         &AcErr::IllSolTypeValue,
//         "Expected ABI decode result to be a String"
//     ))
// }

// Use any_msg_err fn to handle errors
pub fn sol_string_to_str(bs: &Bytes) -> AppResult<String> {
    let ty: DynSolType = "string"
        .parse()
        .map_err(map_err!(&AcErr::IllSolTypeValue, any "SolType(`string`)"))?;
    // .map_err(any_msg_err(&AcErr::IllSolTypeValue, "SolType(`string`)"))?;
    let res = ty
        .abi_decode(&bs)
        .map_err(any_err(&AcErr::EvmSolStrParseErr))?;

    res.as_str().map(|s| s.to_string()).ok_or_else(else_err!(
        &AcErr::IllSolTypeValue,
        "Expected ABI decode result to be a String"
    ))
}

pub fn sol_bytes32_to_str(bs: &Bytes) -> String {
    let len = bs.iter().position(|&b| b == 0).unwrap_or(32);
    let valid_bytes = &bs[..len];
    String::from_utf8_lossy(valid_bytes).to_string()
}

#[cfg(test)]
mod tests {
    use crate::types::sol_abi::ToStrTrait;
    use alloy_primitives::Bytes;

    #[test]
    fn test_sol_string_to_str() {
        let bytes = "0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000034357530000000000000000000000000000000000000000000000000000000000".parse::<Bytes>().unwrap();
        let s = bytes.to_str().unwrap();
        assert_eq!(s, "CWS".to_string());
    }

    #[test]
    fn test_sol_bytes32_to_str() {
        let bytes = "0x4d4b520000000000000000000000000000000000000000000000000000000000"
            .parse::<Bytes>()
            .unwrap();
        let s = bytes.to_str().unwrap();
        assert_eq!(s, "MKR");

        let bytes = "0x4d616b6572000000000000000000000000000000000000000000000000000000"
            .parse::<Bytes>()
            .unwrap();
        let s = bytes.to_str().unwrap();
        assert_eq!(s, "Maker");
    }
}
