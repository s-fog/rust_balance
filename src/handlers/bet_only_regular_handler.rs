use std::sync::Arc;
use axum::{
    response::{ Json as JsonResponse },
    extract::{ Json as JsonExtract }
};
use axum::extract::State;
use serde::{Serialize, Deserialize };
use serde_json::{ Value, json };
use crate::balance_managers::bet_balance_manager::BetBalanceManager;
use crate::money::money::{Currency, Money};
use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct BetOnlyRegularRequest {
    user_id: u64,
    amount: f64,
    currency_id: u16,
    currency_code: String,
    is_game_crash: bool,
}

pub async fn bet_only_regular_handler(
    State(app_state): State<Arc<AppState>>,
    JsonExtract(payload): JsonExtract<BetOnlyRegularRequest>
) -> ()
{
    let bet_balance_manager: BetBalanceManager = BetBalanceManager::new(
        app_state,
        payload.user_id,
        payload.is_game_crash,
        None,
    );

    let money: Money = Money::new(
        Currency::new(payload.currency_id, payload.currency_code),
        payload.amount
    );

    bet_balance_manager.bet_only_regular(money).await;
}