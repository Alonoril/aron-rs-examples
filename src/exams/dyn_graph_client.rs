use crate::error::GrpErr;
use base_infra::result::AppResult;
use base_infra::{else_err, err};
use graphql_client::{QueryBody, Response};
use base_util::http::HttpClient;
use reqwest::{Client, IntoUrl};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub type DynGraphQueryClient = Arc<GraphQueryClient>;

// pub type DynGraphQueryClient = Arc<dyn GraphQueryClientTrait + Send + Sync>;
// pub trait GraphQueryClientTrait {
//     async fn post_gql<T, V, U, S>(&self, query: QueryBody<V>, url: U, biz: S) -> AppResult<T>
//     where
//         U: IntoUrl,
//         T: DeserializeOwned,
//         V: Debug + Serialize,
//         S: Into<String> + Display + ToString;
// }

#[derive(Clone)]
pub struct GraphQueryClient {
    graph_client: Client,
}

impl GraphQueryClient {
    pub fn new() -> Self {
        GraphQueryClient {
            graph_client: HttpClient::default().build_client(),
        }
    }
}

impl GraphQueryClient {
    pub async fn post_gql<T, V, U, S>(&self, query: QueryBody<V>, url: U, biz: S) -> AppResult<T>
    where
        U: IntoUrl,
        T: DeserializeOwned,
        V: Debug + Serialize,
        S: Into<String> + Display + ToString,
    {
        tracing::debug!("biz[{biz}] query: {query:?}");
        let response = self
            .graph_client
            .post(url)
            .json(&query)
            .send()
            .await
            .map_err(base_infra::map_err!(&GrpErr::PostGraphqlErr, biz))?;

        let res = response
            .json::<Response<T>>()
            .await
            .map_err(base_infra::map_err!(&GrpErr::ParseGraphData, biz))?;

        if res.data.is_none() {
            let err = format!("{biz}, reason: {:?}", res.errors);
            return err!(&GrpErr::GraphInnerError, err);
        }

        res.data.ok_or_else(else_err!(&GrpErr::GraphDataEmpty, biz))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Uni3vGraphService {
        client: DynGraphQueryClient,
        /// Uni-v3 theGraph endpoint
        endpoint: String,
    }

    #[test]
    fn test_uni3v_graph_service() {
        let _uni3v_svc = Uni3vGraphService {
            client: Arc::new(GraphQueryClient::new()),
            endpoint: "https://...".to_string(),
        };
    }
}
