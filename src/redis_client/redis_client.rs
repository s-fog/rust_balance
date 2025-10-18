use redis::{Connection, RedisResult};
use dotenv::dotenv;

pub struct RedisClient;

pub trait RedisConnection {
    fn get_connection() -> RedisResult<Connection> {
        dotenv().ok();

        let redis_connection_url = env::var("REDIS_CONNECTION").unwrap();
        let client = redis::Client::open(redis_connection_url).unwrap();

        client.get_connection()
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