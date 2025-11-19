#[macro_export]
macro_rules! map_err_v1 {
    ($code:expr) => {
        |err| {
            tracing::error!("{}, reason: {}", $code, err);
            base_infra::result::AppError::Anyhow($code, anyhow::anyhow!(err))
        }
    };

    ($code:expr, $msg:expr) => {
        |err| {
            tracing::error!("{} {}, reason: {}", $code, $msg, err);
            let msg = ($msg).to_string();
            base_infra::result::AppError::ExtAnyhow($code, msg, anyhow::anyhow!(err))
        }
    };
}

#[macro_export]
macro_rules! map_err_v2 {
    ($code:expr) => {
        |err: impl std::fmt::Debug + std::fmt::Display + Send + 'static| {
            tracing::error!("{}, reason: {}", $code, err);
            base_infra::result::AppError::Anyhow($code, anyhow::anyhow!(err))
        }
    };

    ($code:expr, $msg:expr) => {
        |err: impl std::fmt::Debug + std::fmt::Display + Send + 'static| {
            tracing::error!("{} {}, reason: {}", $code, $msg, err);
            let msg = ($msg).to_string();
            base_infra::result::AppError::ExtAnyhow($code, msg, anyhow::anyhow!(err))
        }
    };
}

pub fn any_err<E>(
    code: &'static base_infra::result::DynErrCode,
) -> impl FnOnce(E) -> base_infra::result::AppError
where
    E: std::error::Error + Into<anyhow::Error>,
{
    move |err: E| {
        tracing::error!("{}, reason: {}", code, err);
        base_infra::result::AppError::Anyhow(code, anyhow::anyhow!(err))
    }
}

pub fn any_msg_err<E>(
    code: &'static base_infra::result::DynErrCode,
    msg: &'static str,
) -> impl FnOnce(E) -> base_infra::result::AppError
where
    E: std::error::Error + Into<anyhow::Error>,
{
    move |err| {
        tracing::error!("{} {}, reason: {}", code, msg, err);
        base_infra::result::AppError::ExtAnyhow(code, msg.to_string(), anyhow::anyhow!(err))
    }
}

base_infra::gen_impl_code_enum! {
    AcErr {
        IllSolTypeValue = ("EST001", "Invalid solidity type"),
        EvmSolStrParseErr = ("EST002", "Failed to parse evm solidity type to rust string"),
    }
}
