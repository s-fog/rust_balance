use redis::{Commands, Connection};
use crate::global::get_redis_client;

pub trait CacheTrait {
    fn get_value(&mut self, key: String) -> Option<String>;
    fn set_value(&mut self, key: String, value: SetValue);
}

pub enum SetValue {
    String(String),
    Float(f64),
    Integer(i32),
}

pub struct CacheRedis {
    connection: Connection
}

impl CacheRedis {
    pub fn new() -> Self {
        Self {
            connection: get_redis_client().get_connection().unwrap()
        }
    }

    pub fn new_with_connection(connection: Connection) -> Self {
        Self {
            connection
        }
    }
}

impl CacheTrait for CacheRedis {

    fn get_value(&mut self, key: String) -> Option<String> {
        self.connection.get(key).ok()
    }

    fn set_value(&mut self, key: String, value: SetValue) {
        let value_string: String = match value {
            SetValue::String(value) => value,
            SetValue::Float(value) => value.to_string(),
            SetValue::Integer(value) => value.to_string(),
        };

        self.connection.set::<String, String, String>(key, value_string).unwrap();
    }

}

#[cfg(test)]
mod tests {
    use crate::global::get_redis_client;
    use super::*;

    fn clear_redis() {
        let mut connection = get_redis_client().get_connection().unwrap();
        redis::cmd("FLUSHDB").query(&mut connection).unwrap()
    }

    fn set_redis_value(key: String, value: String) {
        let mut connection = get_redis_client().get_connection().unwrap();
        redis::cmd("SET").arg(key).arg(value).query(&mut connection).unwrap()
    }

    fn setup() {
        clear_redis();
    }

    #[test]
    fn cache_redis_set_string_value() {
        setup();

        let mut cache_redis = CacheRedis::new_with_connection(get_redis_client().get_connection().unwrap());
        let key = String::from("some-key-1");
        let value = String::from("value");

        assert_eq!(
            (),
            cache_redis.set_value(
                key,
                SetValue::String(value),
            )
        );
    }

    #[test]
    fn cache_redis_set_float_value() {
        setup();

        let mut cache_redis = CacheRedis::new_with_connection(get_redis_client().get_connection().unwrap());
        let key = String::from("some-key-1");
        let value: f64 = 11.5;

        assert_eq!(
            (),
            cache_redis.set_value(
                key,
                SetValue::Float(value),
            )
        );
    }

    #[test]
    fn cache_redis_set_int_value() {
        setup();

        let mut cache_redis = CacheRedis::new_with_connection(get_redis_client().get_connection().unwrap());
        let key = String::from("some-key-1");
        let value: i32 = 11;

        assert_eq!(
            (),
            cache_redis.set_value(
                key,
                SetValue::Integer(value),
            )
        );
    }

    #[test]
    fn cache_redis_get_value_exists() {
        setup();

        let mut cache_redis = CacheRedis::new_with_connection(get_redis_client().get_connection().unwrap());
        let key = String::from("some-key-2");
        let value = String::from("aaa");

        set_redis_value(key.to_string(), value.to_string());

        assert_eq!(value, cache_redis.get_value(key).unwrap());
    }

    #[test]
    fn cache_redis_get_value_not_exists() {
        setup();

        let mut cache_redis = CacheRedis::new_with_connection(get_redis_client().get_connection().unwrap());
        let key = String::from("some-key-3");

        assert_eq!(None, cache_redis.get_value(key));
    }
}