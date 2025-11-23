use std::fmt::Debug;

pub trait MemoryClient: Send + Sync + Debug {
    fn init_cache_key(&self, cache_key: String) -> ();
    fn incr_by_float(&self, cache_key: String, amount: &f64) -> f64;
    fn get_cache_key(&self, user_id: &u64, balance_id: &u64, currency_id: &u16) -> String;
}

pub mod redis_client;
