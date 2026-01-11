use crate::cache::cache::{CacheTrait, CacheRedis, SetValue};
use crate::entities::balance::{Balance, BalanceType};

pub trait BalanceCacheTrait {
    fn get_balance_from_cache(
        &mut self,
        balance_type: &BalanceType,
        user_bonus_id: &Option<u64>,
    ) -> Option<Balance>;

    fn save_balance_to_cache(
        &mut self,
        balance: &Balance,
    ) -> ();

    fn get_balance_cache_key(
        &self,
        balance_type: &BalanceType,
        user_bonus_id: &Option<u64>,
    ) -> String;
}

pub struct BalanceCache {
    cache_client: CacheRedis,
}

impl BalanceCacheTrait for BalanceCache {
    fn get_balance_from_cache(
        &mut self,
        balance_type: &BalanceType,
        user_bonus_id: &Option<u64>,
    ) -> Option<Balance>
    {
        let cache_key = self.get_balance_cache_key(
            &balance_type,
            &user_bonus_id,
        );

        let balance_serialized = self.cache_client.get_value(cache_key);

        if balance_serialized.is_none() {
            return None;
        }

        let balance = serde_json::from_str::<Balance>(&balance_serialized.unwrap());

        match balance {
            Ok(balance) => Option::from(balance),
            Err(_) => return None,
        }
    }

    fn save_balance_to_cache(
        &mut self,
        balance: &Balance,
    )
    {
        let cache_key = self.get_balance_cache_key(
            &balance.get_type(),
            &balance.get_user_bonus_id(),
        );

        self.cache_client.set_value(
                cache_key,
                SetValue::String(serde_json::to_string(&balance).unwrap()),
            );
    }

    fn get_balance_cache_key(
        &self,
        balance_type: &BalanceType,
        user_bonus_id: &Option<u64>,
    ) -> String {
        format!(
            "balance_{}_{:?}",
            balance_type.get_value(),
            user_bonus_id,
        )
    }

}

impl BalanceCache {
    pub fn new() -> Self
    {
        Self {
            cache_client: CacheRedis::new(),
        }
    }
}