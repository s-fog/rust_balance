use redis::{Client, Connection, RedisResult};
use dotenv::dotenv;
use once_cell::sync::Lazy;

pub struct RedisClient;

pub trait RedisConnection {
    fn get_connection() -> RedisResult<Connection> {
        Lazy::new(|| {
            dotenv().ok();

            let redis_connection_url = env::var("REDIS_CONNECTION").unwrap();
            Client::open(redis_connection_url).unwrap()
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