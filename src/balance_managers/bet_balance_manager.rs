use crate::money::money::Money;
use super::balance_manager::balance_manager::{
    BalanceManager,
    BalanceManagerCommitResponse,
};

pub struct BetBalanceManager {
    user_id: u64,
    is_game_crash: bool,
    active_user_bonus_id: Option<u64>,
}

impl BetBalanceManager {
    pub fn new(
        user_id: u64,
        is_game_crash: bool,
        active_user_bonus_id: Option<u64>,
    ) -> Self
    {
        Self {
            user_id,
            is_game_crash,
            active_user_bonus_id,
        }
    }
    pub async fn bet_only_regular(
        &self,
        money: Money,
    ) -> BalanceManagerCommitResponse
    {
        let mut balance_manager: BalanceManager = BalanceManager::new(
            self.user_id,
            None,
            None,
            None,
        );

        balance_manager = balance_manager.decrement_regular_balance(&money);

        if !self.is_game_crash {
            balance_manager = balance_manager.decrement_regular_wager_balance(&money);
        }

        let commit_response = balance_manager.commit().await;

        commit_response
    }
}