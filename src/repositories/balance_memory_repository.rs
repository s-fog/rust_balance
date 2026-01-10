use std::fmt::Debug;
use std::sync::Arc;
use crate::memory_client::MemoryClient;
use crate::money::money::Currency;
use crate::money::money::Money;

pub trait BalanceMemoryRepositoryTrait: Send + Sync + Debug {
    fn increment_balance_amount(
        &self,
        user_id: &u64,
        balance_id: &u64,
        money: &Money,
    ) -> f64;
}

#[derive(Debug)]
pub struct BalanceMemoryRepository {
    pub memory_client: Arc<dyn MemoryClient>
}

impl BalanceMemoryRepository {
    fn get_cache_key(
        &self,
        user_id: &u64,
        balance_id: &u64,
        currency: &Currency,
    ) -> String
    {
        self.memory_client.get_cache_key(user_id, balance_id, &currency.get_id())
    }
}

impl BalanceMemoryRepositoryTrait for BalanceMemoryRepository {
    fn increment_balance_amount(
        &self,
        user_id: &u64,
        balance_id: &u64,
        money: &Money,
    ) -> f64 {
        let cache_key = self.get_cache_key(user_id, balance_id, &money.get_currency());

        self.memory_client.incr_by_float(cache_key, &money.get_amount())
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_client::redis_client::RedisClient;
    use super::*;

    #[test]
    fn increment_balance_amount_with_no_value_in_memory() {
        let redis_client = RedisClient;
        let balance_memory_repository = BalanceMemoryRepository {
            memory_client: Arc::new(redis_client.clone()),
        };
        let user_id: u64 = 1;
        let balance_id: u64 = 1;
        let increment_amount_to: f64 = 120.55;
        let currency = Currency::new(1, String::from("RUB"));
        let money: Money = Money::new(
            currency.clone(),
            increment_amount_to,
        );

        // Clean values before test
        redis_client.remove_value(balance_memory_repository.get_cache_key(&user_id, &balance_id, &currency));

        let new_balance_amount: f64 = balance_memory_repository.increment_balance_amount(&user_id, &balance_id, &money);

        assert_eq!(increment_amount_to, new_balance_amount);
    }
    #[test]
    fn increment_balance_amount_with_value_in_memory() {
        let redis_client = RedisClient;
        let balance_memory_repository = BalanceMemoryRepository {
            memory_client: Arc::new(redis_client.clone()),
        };
        let user_id: u64 = 2;
        let balance_id: u64 = 2;
        let currency = Currency::new(1, String::from("RUB"));

        let initial_amount: f64 = 512_f64;
        let initial_money: Money = Money::new(
            currency.clone(),
            initial_amount,
        );

        let increment_amount_to: f64 = -121.55;
        let money: Money = Money::new(
            currency.clone(),
            increment_amount_to,
        );

        // Clean values before test
        redis_client.remove_value(balance_memory_repository.get_cache_key(&user_id, &balance_id, &currency));

        let new_balance_amount2: f64 = balance_memory_repository.increment_balance_amount(&user_id, &balance_id, &initial_money);
        let new_balance_amount: f64 = balance_memory_repository.increment_balance_amount(&user_id, &balance_id, &money);

        assert_eq!(initial_amount + increment_amount_to, new_balance_amount);
    }
}