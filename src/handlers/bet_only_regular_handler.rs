use serde::{Serialize, Deserialize };
use crate::balance_managers::bet_balance_manager::BetBalanceManager;
use crate::money::money::{Currency, Money};
use axum::{
    extract::Json,
    routing::post,
    handler::Handler,
    Router,
};

use axum::http::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct BetOnlyRegularRequest {
    user_id: u64,
    amount: f64,
    currency_id: u16,
    currency_code: String,
    is_game_crash: bool,
}

pub async fn bet_only_regular_handler(Json(payload): Json<BetOnlyRegularRequest>) -> StatusCode
{
    let bet_balance_manager: BetBalanceManager = BetBalanceManager::new(
        payload.user_id,
        payload.is_game_crash,
        None,
    );

    let money: Money = Money::new(
        Currency::new(
            payload.currency_id,
            payload.currency_code,
        ),
        payload.amount,
    );
    let fut = bet_balance_manager.bet_only_regular(money);
    fn assert_send_future<T: Send>(_: &T) {}
    assert_send_future(&fut);
    //let response = bet_balance_manager.bet_only_regular(money).await;

    StatusCode::OK
}