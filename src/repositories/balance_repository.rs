use sqlx::{AnyPool, MySqlPool};
use crate::entities::balance::{Balance, BalanceType};
use crate::global::{get_redis_client, get_sql_pool};

pub trait BalanceRepositoryTrait {
    async fn get_balance(
        &self,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance;
}

pub struct BalanceRepository;

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
                    .fetch_one(get_sql_pool().await)
                    .await
            }
            None => {
                sqlx::query_as::<_, Balance>(
                        "SELECT * FROM balances WHERE type = ? AND user_bonus_id is null"
                    )
                    .bind(&balance_type)
                    .fetch_one(get_sql_pool().await)
                    .await
            }
        };

        dbg!(&balance_result);

        match balance_result {
            Ok(balance) => {
                balance
            },
            Err(e) => {
                let balance_insert_result = sqlx::query(
                        "INSERT INTO balances (type, user_bonus_id) VALUES (?, ?)"
                    )
                    .bind(&balance_type)
                    .bind(&user_bonus_id)
                    .execute(get_sql_pool().await)
                    .await
                    .unwrap();

                let id: u64 = balance_insert_result.last_insert_id();
                dbg!(&id);

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
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn get_balance_balance_not_exists() {
        let balance_type = BalanceType::Regular;
        let user_bonus_id = None;

        let balance_repository = BalanceRepository;
        let balance = balance_repository.get_balance(balance_type, user_bonus_id).await;

        assert_eq!(true, balance.id > 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn get_balance_balance_exist() {
        let balance_type = BalanceType::Bonus;
        let user_bonus_id = Some(5);

        sqlx::query("INSERT INTO balances (type, user_bonus_id) VALUES (?, ?)")
            .bind(&balance_type)
            .bind(&user_bonus_id)
            .execute(get_sql_pool().await)
            .await;

        let balance_repository = BalanceRepository;
        let balance = balance_repository.get_balance(balance_type, user_bonus_id).await;

        assert_eq!(true, balance.id > 0);
    }
}