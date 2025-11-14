pub mod database_connection {
    use crate::env_load;
    use sqlx::mysql::{ MySqlPool, MySqlPoolOptions };
    use sqlx::pool::Pool;
    use once_cell::sync::Lazy;
    use tokio::runtime::Handle;

    pub struct Mysql {
        pub pool: MySqlPool,
    }

    impl Mysql {
        pub async fn get_pool() -> Self {
            let lazy_pool: Lazy<MySqlPool> = Lazy::new(|| {
                Handle::current().block_on(async {
                    MySqlPoolOptions::new()
                        .min_connections(1)
                        .max_connections(10)
                        .connect("mysql://user:pass@localhost/test_db")
                        .await
                        .unwrap()
                })
            });

            Self { pool: Lazy::into_value(lazy_pool).unwrap() }
        }
    }
}