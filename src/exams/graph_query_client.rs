use crate::error::GrpErr;
use base_infra::result::AppResult;
use base_infra::{else_err, err, map_err};
use graphql_client::{QueryBody, Response};
use base_util::Client;
use base_util::http::HttpClient;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use std::fmt::Display;

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

    pub async fn post_gql<T, V, U, S>(&self, query: QueryBody<V>, url: U, biz: S) -> AppResult<T>
    where
        U: IntoUrl,
        V: serde::Serialize,
        // T: serde::Deserialize,
        // T: for<'a> serde::Deserialize<'a>,
        T: DeserializeOwned,
        S: Into<String> + ToString + Display,
    {
        let res = self
            .graph_client
            .post(url)
            .json(&query)
            .send()
            .await
            .map_err(map_err!(&GrpErr::PostGraphqlErr, biz))?;

        let res = res
            .json::<Response<T>>()
            .await
            .map_err(map_err!(&GrpErr::ParseGraphData, biz))?;

        if res.data.is_none() {
            let err = format!("{biz} {:?}", res.errors);
            return err!(&GrpErr::GraphInnerError, err);
        }

        res.data.ok_or_else(else_err!(&GrpErr::GraphDataEmpty, biz))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_gql() {
        let client = GraphQueryClient::new();
    }
}
