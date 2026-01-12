mod balance_getter;
mod balance_parts;
mod balance_and_amount;

pub mod balance_manager {
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

    pub struct BalanceManager {
        user_id: u64,
        active_user_bonus_id: Option<u64>,
        changing_user_bonus_id: Option<u64>,
        balance_parts_before: BalanceParts,
        balance_parts_after: BalanceParts,
        balance_parts_changes: BalanceParts,
        balance_getter_service: Box<dyn BalanceGetter + Send + Sync>
    }

    impl BalanceManager {
        pub fn new(
            user_id: u64,
            active_user_bonus_id: Option<u64>,
            changing_user_bonus_id: Option<u64>,
            balance_getter_service: Option<Box<dyn BalanceGetter + Send + Sync>>,
        ) -> Self
        {
            let bgs = match balance_getter_service {
                None => Box::new(BalanceGetterService::new()),
                Some(bgs) => bgs,
            };

            Self {
                user_id,
                active_user_bonus_id,
                changing_user_bonus_id,
                balance_parts_before: BalanceParts::empty(),
                balance_parts_after: BalanceParts::empty(),
                balance_parts_changes: BalanceParts::empty(),
                balance_getter_service: bgs
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
