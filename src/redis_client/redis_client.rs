use redis::{Client, Connection, RedisResult};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use crate::env_load::env_load::get_env_var;


pub struct RedisClient;

pub trait RedisConnection {
    fn get_connection() -> RedisResult<Connection> {
        Lazy::new(|| {
            let connection_url = get_env_var(&String::from("REDIS_CONNECTION"));

            Client::open(connection_url).unwrap()
        }).get_connection()
    }
}

impl RedisConnection for RedisClient {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_redis_connection() {
        let redis_result = RedisClient::get_connection();

        assert_eq!(true, redis_result.is_ok());
    }
}