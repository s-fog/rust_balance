pub mod balance_getter {
    use std::sync::Arc;
    use crate::entities::balance::{Balance, BalanceType};
    use crate::state::AppState;

    pub async fn get_balance(
        app_state: Arc<AppState>,
        balance_type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance
    {
        println!("get_balance");
        app_state.balance_repository.get_balance(
            balance_type,
            user_bonus_id,
        ).await
    }
}
