use std::sync::Arc;
use axum::{
    response::{ Json as JsonResponse },
    extract::{ Json as JsonExtract }
};
use axum::extract::State;
use serde::{Serialize, Deserialize };
use serde_json::{ Value, json };
use crate::handlers::traits::TransformToBalanceManager;
use crate::services::balance_manager::BalanceManager;
use crate::state::AppState;
use crate::structs::balance_and_amount::BalanceAndAmount;
use crate::structs::balance_parts::BalanceParts;
use crate::structs::money::Money;

#[derive(Serialize, Deserialize, Debug)]
pub struct BetOnlyRegularRequest {
    user_id: u64,
    money: Option<Money>,
}

impl TransformToBalanceManager for BetOnlyRegularRequest {
    fn transform(&self, app_state: Arc<AppState>) -> BalanceManager {
        let mut balance_parts_changes = BalanceParts::new(
            None,
            None,
            None,
            None,
        );

        BalanceManager::new(
            app_state,
            self.user_id,
            None,
        )
    }
}

pub async fn bet_only_regular(
    State(app_state): State<Arc<AppState>>,
    JsonExtract(payload): JsonExtract<BetOnlyRegularRequest>
) -> ()
{
    let balance_manager: BalanceManager = payload.transform(app_state);

    dbg!(&balance_manager);

}