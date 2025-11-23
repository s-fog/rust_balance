use serde::{ Deserialize, Serialize };
use super::balance_and_amount::BalanceAndAmount;

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
}

