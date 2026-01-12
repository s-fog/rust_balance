use std::ops::Deref;
use std::sync::Arc;
use sqlx::MySqlPool;
use crate::entities::balance::{Balance, BalanceType};
use crate::global::get_sql_pool_sync;

#[async_trait::async_trait]
pub trait BalanceRepositoryTrait: Send + Sync {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance;
}

pub struct BalanceRepository {
    sql_pool: Arc<MySqlPool>,
}

impl BalanceRepository {
    pub fn new() -> Self {
        Self {
            sql_pool: get_sql_pool_sync()
        }
    }

    pub fn new_for_tests(pool: Arc<MySqlPool>) -> Self {
        Self {
            sql_pool: pool
        }
    }
}

#[async_trait::async_trait]
impl BalanceRepositoryTrait for BalanceRepository {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance {
        let balance_result = match user_bonus_id {
            Some(_) => {
                sqlx::query_as::<_, Balance>(
                    "SELECT * FROM balances WHERE type = ? AND user_bonus_id = ?"
                )
                    .bind(&balance_type)
                    .bind(&user_bonus_id)
                    .fetch_one(self.sql_pool.clone().deref())
                    .await
            }
            None => {
                sqlx::query_as::<_, Balance>(
                    "SELECT * FROM balances WHERE type = ? AND user_bonus_id is null"
                )
                    .bind(&balance_type)
                    .fetch_one(self.sql_pool.clone().deref())
                    .await
            }
        };

        match balance_result {
            Ok(balance) => {
                balance
            },
            Err(_e) => {
                let balance_insert_result = sqlx::query(
                    "INSERT INTO balances (type, user_bonus_id) VALUES (?, ?)"
                )
                    .bind(&balance_type)
                    .bind(&user_bonus_id)
                    .execute(self.sql_pool.clone().deref())
                    .await
                    .unwrap();

                let id: u64 = balance_insert_result.last_insert_id();

                Balance::new(
                    id,
                    balance_type,
                    user_bonus_id,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use sqlx::mysql::MySqlPoolOptions;
    use crate::env_load::env_load::get_env_var;
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn get_balance_balance_not_exists() {
        let balance_type = BalanceType::Regular;
        let user_bonus_id = None;
        let sql_pool = Arc::new(MySqlPoolOptions::new()
            .min_connections(1)
            .max_connections(1)
            .connect(&get_env_var(&String::from("DATABASE_URL")))
            .await
            .unwrap());

        // Prepare database
        sqlx::query("DELETE FROM balances WHERE type = ? and user_bonus_id IS NULL")
            .bind(&balance_type)
            .execute(sql_pool.clone().deref())
            .await;
        // Prepare database END

        let balance_repository = BalanceRepository::new_for_tests(sql_pool);
        let balance = balance_repository.get_balance(balance_type, user_bonus_id).await;

        assert_eq!(true, balance.get_id() > 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn get_balance_balance_exist() {
        let balance_type = BalanceType::Bonus;
        let user_bonus_id = Some(5);
        let sql_pool = Arc::new(MySqlPoolOptions::new()
            .min_connections(1)
            .max_connections(1)
            .connect(&get_env_var(&String::from("DATABASE_URL")))
            .await
            .unwrap());

        // Prepare database
        sqlx::query("DELETE FROM balances WHERE type = ? and user_bonus_id = ?")
            .bind(&balance_type)
            .bind(&user_bonus_id)
            .execute(sql_pool.clone().deref())
            .await;
        sqlx::query("INSERT INTO balances (type, user_bonus_id) VALUES (?, ?)")
            .bind(&balance_type)
            .bind(&user_bonus_id)
            .execute(sql_pool.clone().deref())
            .await;
        // Prepare database END


        let balance_repository = BalanceRepository::new_for_tests(sql_pool);
        let balance = balance_repository.get_balance(balance_type, user_bonus_id).await;

        assert_eq!(true, balance.get_id() > 0);
    }
}