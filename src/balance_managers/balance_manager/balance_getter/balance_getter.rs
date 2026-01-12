use std::sync::Arc;
use super::balance_cache::balance_cache::{
    BalanceCacheTrait,
    BalanceCache,
};
use super::balance_repository::balance_repository::{
    BalanceRepositoryTrait,
    BalanceRepository,
};
use crate::entities::balance::{
    Balance,
    BalanceType,
};

#[async_trait::async_trait]
pub trait BalanceGetter {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance;
}

pub struct BalanceGetterService {
    balance_repository: Arc<dyn BalanceRepositoryTrait>,
    balance_cache: Arc<dyn BalanceCacheTrait>,
}

impl BalanceGetterService {
    pub fn new() -> Self
    {
        Self {
            balance_repository: Arc::new(BalanceRepository::new()),
            balance_cache: Arc::new(BalanceCache::new()),
        }
    }
}

#[async_trait::async_trait]
impl BalanceGetter for BalanceGetterService {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance
    {
        let balance_from_cache: Option<Balance> = self.balance_cache.get_balance_from_cache(
            &balance_type,
            &user_bonus_id,
        );

        if balance_from_cache.is_some() {
            return balance_from_cache.unwrap();
        }

        let balance = self.balance_repository.get_balance(
            balance_type,
            user_bonus_id,
        ).await;

        self.balance_cache.save_balance_to_cache(&balance);

        balance
    }
}