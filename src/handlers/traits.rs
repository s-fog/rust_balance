use std::sync::Arc;
use crate::services::balance_manager::BalanceManager;
use crate::state::AppState;

pub trait TransformToBalanceManager {
    fn transform(&self, app_state: Arc<AppState>) -> BalanceManager;
}