//#![cfg_attr(feature = "local", allow(dead_code, unused))]
#![allow(dead_code, unused)]
mod memory_client;
mod env_load;
mod global;
mod state;
mod structs;
mod entities;
mod repositories;
mod services;
mod handlers;

use std::sync::Arc;
use axum::{ Router, routing::{ post } };
use crate::global::get_sql_pool;
use crate::handlers::bet_only_regular::bet_only_regular;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    get_sql_pool().await;
    let app_state = Arc::new(AppState::make_real());

    let app = Router::new()
        .route(
            "/bet/only-regular",
            post(bet_only_regular)
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
