use redis::{ Commands };
use crate::memory_client::MemoryClient;
use crate::global::get_redis_client;

#[derive(Clone, Debug)]
pub struct RedisClient;

impl RedisClient {
    pub fn remove_value(&self, cache_key: String) -> () {
        let mut connection = get_redis_client().get_connection().unwrap();

        connection.del::<String, ()>(cache_key).unwrap()
    }
}

impl MemoryClient for RedisClient {
    fn init_cache_key(&self, cache_key: String) -> () {
        let mut connection = get_redis_client().get_connection().unwrap();

        connection.set_nx::<String, f64, f64>(cache_key, 0_f64).unwrap();
    }

    fn incr_by_float(&self, cache_key: String, amount: &f64) -> f64 {
        let mut connection = get_redis_client().get_connection().unwrap();

        connection.incr(cache_key, amount).unwrap()
    }

    fn get_cache_key(&self, user_id: &u64, balance_id: &u64, currency_id: &u16) -> String {
        format!(
            "balance-for-user-{}-balance_id-{}-currency-{}",
            user_id,
            balance_id,
            currency_id,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_redis_connection() {
        let connection = get_redis_client().get_connection();

        assert_eq!(true, connection.is_ok());
    }
}