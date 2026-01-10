use serde::{ Deserialize, Serialize };
use crate::balance_managers::balance_manager::balance_and_amount::BalanceAndAmount;

#[derive(Deserialize, Serialize, Debug)]
pub struct BalanceParts {
    regular_balance_and_amount: Option<BalanceAndAmount>,
    regular_wager_balance_and_amount: Option<BalanceAndAmount>,
    bonus_balance_and_amount: Option<BalanceAndAmount>,
    bonus_wager_balance_and_amount: Option<BalanceAndAmount>,
}

impl BalanceParts {
    pub fn new(
        regular_balance_and_amount: Option<BalanceAndAmount>,
        regular_wager_balance_and_amount: Option<BalanceAndAmount>,
        bonus_balance_and_amount: Option<BalanceAndAmount>,
        bonus_wager_balance_and_amount: Option<BalanceAndAmount>,
    ) -> BalanceParts
    {
        BalanceParts {
            regular_balance_and_amount,
            regular_wager_balance_and_amount,
            bonus_balance_and_amount,
            bonus_wager_balance_and_amount,
        }
    }

    pub fn empty() -> BalanceParts
    {
        BalanceParts {
            regular_balance_and_amount: None,
            regular_wager_balance_and_amount: None,
            bonus_balance_and_amount: None,
            bonus_wager_balance_and_amount: None,
        }
    }

    pub fn is_empty(&self) -> bool
    {
        self.regular_balance_and_amount.is_none()
            && self.regular_wager_balance_and_amount.is_none()
            && self.bonus_balance_and_amount.is_none()
            && self.bonus_wager_balance_and_amount.is_none()
    }
}

