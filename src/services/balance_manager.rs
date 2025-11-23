use std::sync::Arc;
use serde::{Deserialize, Serialize };
use crate::state::AppState;
use crate::structs::balance_parts::BalanceParts;

#[derive(Debug)]
pub struct BalanceManager {
    app_state: Arc<AppState>,
    user_id: u64,
    user_bonus_id: Option<u64>,
    balance_parts_before: Option<BalanceParts>,
    balance_parts_after: Option<BalanceParts>,
    balance_parts_changes: Option<BalanceParts>,
}

impl BalanceManager {
    pub fn new(
        app_state: Arc<AppState>,
        user_id: u64,
        user_bonus_id: Option<u64>,
    ) -> BalanceManager {
        BalanceManager {
            app_state,
            user_id,
            user_bonus_id,
            balance_parts_before: None,
            balance_parts_after: None,
            balance_parts_changes: None,
        }
    }
}

