mod balance_getter;
mod balance_parts;
mod balance_and_amount;

pub mod balance_manager {
    use std::sync::Arc;
    use super::balance_getter::balance_getter::{BalanceGetter, BalanceGetterService};
    use super::balance_parts::BalanceParts;
    use crate::entities::balance::{Balance, BalanceType};
    use crate::money::money::Money;
    use crate::state::AppState;

    pub struct BalanceManagerCommitResponse {
        is_balance_enough: bool,
        balance_parts_before: BalanceParts,
        balance_parts_after: BalanceParts,
    }

    impl BalanceManagerCommitResponse {
        pub fn make_empty() -> Self
        {
            Self {
                is_balance_enough: true,
                balance_parts_before: BalanceParts::empty(),
                balance_parts_after: BalanceParts::empty(),
            }
        }
    }

    #[derive(Debug)]
    pub struct BalanceManager {
        pub app_state: Arc<AppState>,
        pub user_id: u64,
        pub active_user_bonus_id: Option<u64>,
        pub changing_user_bonus_id: Option<u64>,
        pub balance_parts_before: BalanceParts,
        pub balance_parts_after: BalanceParts,
        pub balance_parts_changes: BalanceParts,
    }

    impl BalanceManager {
        pub fn new(
            app_state: Arc<AppState>,
            user_id: u64,
            active_user_bonus_id: Option<u64>,
            changing_user_bonus_id: Option<u64>,
        ) -> Self
        {
            Self {
                app_state,
                user_id,
                active_user_bonus_id,
                changing_user_bonus_id,
                balance_parts_before: BalanceParts::empty(),
                balance_parts_after: BalanceParts::empty(),
                balance_parts_changes: BalanceParts::empty(),
            }
        }

        pub fn decrement_regular_balance(
            self,
            amount: &Money,
        ) -> Self
        {
            self
        }

        pub fn decrement_regular_wager_balance(
            self,
            amount: &Money,
        ) -> Self
        {
            self
        }

        pub async fn commit(
            &self,
        ) -> BalanceManagerCommitResponse
        {
            if self.balance_parts_changes.is_empty() {
                return BalanceManagerCommitResponse::make_empty();
            }

            let mut balance_getter = BalanceGetterService::new();

            let balance: Balance = balance_getter.get_balance(
                BalanceType::Regular,
                None,
            ).await;

            BalanceManagerCommitResponse::make_empty()
        }
    }

}
