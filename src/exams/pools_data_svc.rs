use crate::exams::graph_client_mgr::GraphClientManager;
use alloy_primitives::{Address, U256};
use base_infra::result::AppResult;
use futures::future::{try_join_all, BoxFuture};
use std::sync::Arc;

pub type DynPoolState = Arc<dyn PoolStateTrait + Send + Sync>;
#[async_trait::async_trait]
pub trait PoolStateTrait {
    async fn calc_expected_amount_out(
        &mut self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
    ) -> AppResult<U256>;

    async fn reset(&mut self) -> AppResult<()>;

    async fn update(
        &mut self,
        token_in: Address,
        amount_in: U256,
        amount_out: U256,
    ) -> AppResult<()>;
}
#[derive(Clone)]
pub struct PoolsDataService {
    pub graph_client: GraphClientManager,
}

impl PoolsDataService {
    // // Code that cannot be compiled
    // pub async fn fetch_pools_data(
    //     &self,
    //     token0: Address,
    //     token1: Address,
    // ) -> AppResult<Vec<DynPoolState>> {
    //     let (mut futures, num_pools) = (Vec::with_capacity(4), 5);
    //     // call Graph API
    //     futures.push(self.get_top_pools(num_pools));
    //     futures.push(self.get_pools_with_token(token0, num_pools));
    //     futures.push(self.get_pools_with_token(token1, num_pools));
    //     futures.push(self.get_pools_with_token_pair(token0, token1, num_pools));
    //
    //     // Parallel execution
    //     try_join_all(futures).await?;
    //
    //     // Get the state of all pools from the chain
    //     self.get_pools_state().await
    // }

    pub async fn fetch_pools_data(
        &self,
        token0: Address,
        token1: Address,
    ) -> AppResult<Vec<DynPoolState>> {
        let num_pools = 5;
        let mut futures: Vec<BoxFuture<'_, AppResult<()>>> = Vec::with_capacity(4);
        // call Graph API
        futures.push(Box::pin(self.get_top_pools(num_pools)));
        futures.push(Box::pin(self.get_pools_with_token(token0, num_pools)));
        futures.push(Box::pin(self.get_pools_with_token(token1, num_pools)));
        futures.push(Box::pin(
            self.get_pools_with_token_pair(token0, token1, num_pools),
        ));

        // Parallel execution
        try_join_all(futures).await?;

        // Get the state of all pools from the chain
        self.get_pools_state().await
    }

    async fn get_pools_state(&self) -> AppResult<Vec<DynPoolState>> {
        todo!("Just to show if it can compile, the real business logic is removed")
    }

    async fn get_top_pools(&self, num_pools: usize) -> AppResult<()> {
        // Just to show if it can compile, the real business logic is removed
        Ok(())
    }

    async fn get_pools_with_token(&self, token: Address, num_pools: usize) -> AppResult<()> {
        // Just to show if it can compile, the real business logic is removed
        Ok(())
    }

    async fn get_pools_with_token_pair(
        &self,
        token0: Address,
        token1: Address,
        num_pools: usize,
    ) -> AppResult<()> {
        // Just to show if it can compile, the real business logic is removed
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::exams::graph_client_mgr::GraphClientManager;
    use crate::exams::pools_data_svc::PoolsDataService;
    use crate::types::dex_types::DexId;
    use alloy_primitives::Address;
    use base_infra::result::AppResult;

    #[tokio::test]
    async fn test_pools_data_service() -> AppResult<()> {
        let dex_ids = vec![DexId::UniV2, DexId::UniV3];
        let pools_data_service = PoolsDataService {
            graph_client: GraphClientManager::new(&dex_ids)?,
        };

        let token0 = Address::from([0x0; 20]);
        let token1 = Address::from([0x1; 20]);
        let _ = pools_data_service.fetch_pools_data(token0, token1).await;

        Ok(())
    }
}
