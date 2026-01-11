use std::sync::Arc;
use crate::memory_client::redis_client::RedisClient;
use crate::repositories::balance_memory_repository::{BalanceMemoryRepository, BalanceMemoryRepositoryTrait};

#[derive(Debug)]
pub struct AppState {
    pub balance_memory_repository: Arc<dyn BalanceMemoryRepositoryTrait>,
    pub balance_memory_service: (),
}

impl AppState {
    pub fn make_real() -> AppState {
        let balance_memory_repository = BalanceMemoryRepository {
            memory_client: Arc::new(RedisClient),
        };

        AppState {
            balance_memory_repository: Arc::new(balance_memory_repository),
            balance_memory_service: (),
        }
    }
}