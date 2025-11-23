use sqlx::mysql::{MySqlPool, MySqlPoolOptions };
use tokio::sync::OnceCell as TokioOnceCell;
use once_cell::sync::OnceCell;
use redis::{ Client };
use crate::env_load::env_load::get_env_var;

static SQL_POOL: TokioOnceCell<MySqlPool> = TokioOnceCell::const_new();
static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();

pub async fn get_sql_pool() -> &'static MySqlPool
{
    SQL_POOL.get_or_init(|| async {
        let url: String = get_env_var(&String::from("DATABASE_URL"));

        println!("{}", &url);

        MySqlPoolOptions::new()
            .min_connections(1)
            .max_connections(10)
            .connect(&url)
            .await
            .unwrap()
    }).await
}

pub fn get_redis_client() -> &'static Client
{
    REDIS_CLIENT.get_or_init(|| {
        let connection_url = get_env_var(&String::from("REDIS_CONNECTION"));

        Client::open(connection_url).unwrap()
    })
}