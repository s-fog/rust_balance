use std::sync::Arc;
use crate::global::{get_sql_pool, get_sql_pool_sync};
use crate::memory_client::redis_client::RedisClient;
use crate::repositories::balance_memory_repository::{BalanceMemoryRepository, BalanceMemoryRepositoryTrait};
use crate::repositories::balance_repository::{BalanceRepository, BalanceRepositoryTrait};

#[derive(Debug)]
pub struct AppState {
    pub balance_memory_repository: Arc<dyn BalanceMemoryRepositoryTrait>,
    pub balance_memory_service: (),
    pub balance_repository: Arc<dyn BalanceRepositoryTrait>,
}

impl AppState {
    pub fn make_real() -> AppState {
        let balance_memory_repository = BalanceMemoryRepository {
            memory_client: Arc::new(RedisClient),
        };
        let balance_repository = BalanceRepository {
            sql_pool: get_sql_pool_sync()
        };

        AppState {
            balance_memory_repository: Arc::new(balance_memory_repository),
            balance_memory_service: (),
            balance_repository: Arc::new(balance_repository),
        }
    }
}