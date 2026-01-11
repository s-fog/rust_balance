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
use crate::state::AppState;
use crate::cache::cache::{
    CacheRedis,
    SetValue,
};
use crate::global::get_redis_client;

pub trait BalanceGetter {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance;
}

pub struct BalanceGetterService<BalanceCacheTrait, BalanceRepositoryTrait> {
    balance_repository: BalanceRepositoryTrait,
    balance_cache: BalanceCacheTrait,
}

impl BalanceGetterService<BalanceCache, BalanceRepository<'_>> {
    pub fn new() -> Self
    {
        Self {
            balance_repository: BalanceRepository::new(),
            balance_cache: BalanceCache::new(),
        }
    }
}

impl<C: BalanceCacheTrait, R: BalanceRepositoryTrait> BalanceGetter for BalanceGetterService<C, R> {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance {
        let balance_from_cache: Option<Balance> = self.balance_cache.get_balance_from_cache(
            &balance_type,
            &user_bonus_id,
        );

        if (balance_from_cache.is_some()) {
            return balance_from_cache.unwrap();
        }

        let balance = self.balance_repository.get_balance(
            balance_type,
            user_bonus_id,
        ).await;

        self.balance_cache.save_balance_to_cache(&balance, );

        balance
    }
}