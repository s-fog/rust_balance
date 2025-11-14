
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_redis_connection() {
        let redis_result = RedisClient::get_connection();

        assert_eq!(true, redis_result.is_ok());
    }
}