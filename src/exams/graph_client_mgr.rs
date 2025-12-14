use crate::error::GrpErr;
use crate::exams::dyn_graph_client::{DynGraphQueryClient, GraphQueryClient};
use crate::types::dex_types::{DexId, PairsResponse, PoolInfo, PoolsResponse};
use base_infra::err;
use base_infra::result::AppResult;
use graphql_client::QueryBody;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

pub type DynDexGraphClient = Arc<dyn DexGraphClientTrait + Send + Sync + 'static>;

#[async_trait::async_trait]
pub trait DexGraphClientTrait {
    async fn get_top_pools(&self, num_pools: usize) -> AppResult<Vec<PoolInfo>>;
}

pub struct GraphClientManager {
    graph_clients: HashMap<DexId, DynDexGraphClient>,
}

impl GraphClientManager {
    // /// Cannot use into_iter().map() and collect to convert to hashmap
    // pub fn new(dex_ids: Vec<DexId>) -> AppResult<Self> {
    //     let client = Arc::new(GraphQueryClient::new());
    //
    //     let graph_clients = dex_ids
    //         .into_iter()
    //         .map(|id| {
    //             let client = if id.eq(&DexId::UniV2) {
    //                 let url =
    //                     "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2".to_string();
    //                 Ok(Arc::new(Uni2vGraphService::new(url, client.clone())) as DynDexGraphClient)
    //             } else if id.eq(&DexId::UniV3) {
    //                 let url =
    //                     "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3".to_string();
    //                 Ok(Arc::new(Uni2vGraphService::new(url, client.clone())) as DynDexGraphClient)
    //             } else {
    //                 err!(&GrpErr::UnsupportedDexId, id)
    //             };
    //             (id, client)
    //         })
    //         .collect::<AppResult<HashMap<DexId, DynDexGraphClient>>>()?;
    //
    //     Ok(GraphClientManager { graph_clients })
    // }

    // /// Create a hashmap directly using the for loop traversal
    pub fn new(dex_ids: &Vec<DexId>) -> AppResult<Self> {
        let client = Arc::new(GraphQueryClient::new());
        let mut graph_clients = HashMap::with_capacity(dex_ids.len());

        for id in dex_ids {
            let client = if id.eq(&DexId::UniV2) {
                let url = "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2".to_string();
                Arc::new(Uni2vGraphService::new(url, client.clone())) as DynDexGraphClient
            } else if id.eq(&DexId::UniV3) {
                let url = "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3".to_string();
                Arc::new(Uni3vGraphService::new(url, client.clone())) as DynDexGraphClient
            } else {
                return err!(&GrpErr::UnsupportedDexId, id);
            };
            graph_clients.insert(*id, client);
        }
        Ok(GraphClientManager { graph_clients })
    }
}

pub struct Uni2vGraphService {
    client: DynGraphQueryClient,
    /// Uni-v2 theGraph endpoint
    endpoint: String,
}

impl Uni2vGraphService {
    pub fn new(endpoint: String, client: DynGraphQueryClient) -> Self {
        Self { client, endpoint }
    }
    fn build_top_pools_query(num_pools: usize) -> QueryBody<Value> {
        let query = r#"
query topPools($numPools: Int!) {
  pairs(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {
	id
	volumeUSD
	token0 {
	  id
	  name
	  symbol
	  decimals
	}
	token1 {
	  id
	  name
	  symbol
	  decimals
	}
  }
}"#;

        QueryBody {
            variables: json!({"numPools": num_pools}),
            query: &query,
            operation_name: "topPools",
        }
    }
}
#[async_trait::async_trait]
impl DexGraphClientTrait for Uni2vGraphService {
    async fn get_top_pools(&self, num_pools: usize) -> AppResult<Vec<PoolInfo>> {
        let query = Self::build_top_pools_query(num_pools);
        let (url, biz) = (&self.endpoint, "ethereum uni-v2 top_pools");
        let res: PairsResponse = self.client.post_gql(query, url, biz).await?;
        Ok(res.pairs)
    }
}

pub struct Uni3vGraphService {
    client: DynGraphQueryClient,
    /// Uni-v3 theGraph endpoint
    endpoint: String,
}

impl Uni3vGraphService {
    pub fn new(endpoint: String, client: DynGraphQueryClient) -> Self {
        Self { client, endpoint }
    }
    fn build_top_pools_query(num_pools: usize) -> QueryBody<Value> {
        let query = r#"
	query topPools($numPools: Int!) {
	  pools(first: $numPools, orderDirection: desc, orderBy: volumeUSD) {
	    id
	    volumeUSD
	    token0 {
	      id
	      name
	  symbol
	      decimals
	    }
	    token1 {
	      id
	      name
	  symbol
	      decimals
	    }
	  }
	}"#;

        QueryBody {
            variables: json!({"numPools": num_pools}),
            query: &query,
            operation_name: "topPools",
        }
    }
}
#[async_trait::async_trait]
impl DexGraphClientTrait for Uni3vGraphService {
    async fn get_top_pools(&self, num_pools: usize) -> AppResult<Vec<PoolInfo>> {
        let query = Self::build_top_pools_query(num_pools);
        let (url, biz) = (&self.endpoint, "ethereum uni-v2 top_pools");
        let res: PoolsResponse = self.client.post_gql(query, url, biz).await?;
        Ok(res.pools)
    }
}

#[cfg(test)]
mod tests {
    use crate::exams::graph_client_mgr::GraphClientManager;
    use crate::types::dex_types::DexId;
    use base_infra::result::AppResult;

    #[test]
    fn it_works() -> AppResult<()> {
        let dex_ids = vec![DexId::UniV2, DexId::UniV3];
        let mgr = GraphClientManager::new(&dex_ids)?;
        assert_eq!(mgr.graph_clients.len(), 2);
        Ok(())
    }
}
