use crate::exams::graph_query_client::GraphQueryClient;
use crate::types::dex_types::{DexId, PoolInfo, PoolsResponse};
use base_infra::result::AppResult;
use graphql_client::QueryBody;
use serde_json::{Value, json};

pub struct Uni3vGraphService {
    /// Uni-v3 theGraph endpoint
    endpoint: String,
    client: GraphQueryClient,
}
impl Uni3vGraphService {
    // async fn get_top_pools(&self, num_pools: i32) -> AppResult<Vec<PoolInfo>> {
    //     let query = Self::build_top_pools_query(num_pools);
    //     let (url, biz) = (&self.endpoint, "Ethereum uni-v3 top_pools");
    //     let res: PoolsResponse = self.client.post_gql(query, url, biz).await?;
    //
    //     let mut pools = res.pools;
    //     for pool in pools.iter_mut() {
    //         pool.with_dex_id(DexId::UniV3);
    //     }
    //     Ok(pools)
    // }

    async fn get_top_pools(&self, num_pools: i32) -> AppResult<Vec<PoolInfo>> {
        let query = Self::build_top_pools_query(num_pools);
        let (url, biz) = (&self.endpoint, "Ethereum uni-v3 top_pools");
        let res: PoolsResponse = self.client.post_gql(query, url, biz).await?;

        let mut pools = Vec::with_capacity(res.pools.len());
        for pool in res.pools {
            let p = pool.with_dex_id(DexId::UniV3);
            pools.push(p);
        }
        Ok(pools)
    }

    fn build_top_pools_query(num_pools: i32) -> QueryBody<Value> {
        let query = r#"
query topPools($numPools: Int!) {
  pairs(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {
    id
    volumeUSD
    token0 {
      id
      name
      decimals
    }
    token1 {
      id
      name
      decimals
    }
  }
}"#;
        QueryBody {
            variables: json!({"numPools": num_pools}),
            query,
            operation_name: "topPools",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_top_pools() {
        let service = Uni3vGraphService {
            endpoint: "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3".to_string(),
            client: GraphQueryClient::new(),
        };
        let pools = service.get_top_pools(10).await.unwrap();
        println!("{:?}", pools);
    }
}
