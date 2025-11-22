#[macro_export]
macro_rules! map_err {
    // 无 msg：旧逻辑，用闭包包装（必须用闭包）
    ($code:expr) => {
        |err| {
            tracing::error!("{}, reason: {}", $code, err);
            base_infra::result::AppError::Anyhow($code, anyhow::anyhow!(err))
        }
    };

    // 有 msg：直接返回 any_msg_err，交给它决定错误类型
    ($code:expr, $msg:expr) => {{
        tracing::error!("{} {}", $code, $msg);
        $crate::error::any_msg_err($code, $msg)
    }};
}

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
        tracing::error!("{}, reason: {}", code.code(), err);
        base_infra::result::AppError::ExtAnyhow(code, msg.to_string(), anyhow::anyhow!(err))
    }
}

base_infra::gen_impl_code_enum! {
    AcErr {
        IllSolTypeValue = ("EST001", "Invalid solidity type"),
        EvmSolStrParseErr = ("EST002", "Failed to parse evm solidity type to rust string"),
    }
}

base_infra::gen_impl_code_enum! {
    DBErr {
        InitDbPoolErr = ("DBP001", "error while initializing the database connection pool"),
    }
}

base_infra::gen_impl_code_enum! {
	GrpErr {
		PostGraphqlErr = ("GRP001", "Failed to post graphql data for"),
		ParseGraphData = ("GRP002", "Failed to parse graph data for"),
		GraphDataEmpty = ("GRP003", "Graph data is empty for"),
		GraphInnerError = ("GRP004", "Graph internal error for"),
	}
}

