//#![cfg_attr(feature = "local", allow(dead_code, unused))]
#![allow(dead_code, unused)]
mod memory_client;
mod env_load;
mod global;
mod state;
mod money;
mod entities;
mod repositories;
mod balance_managers;
mod handlers;

use std::sync::Arc;
use axum::{ Router, routing::{ post } };
use crate::global::get_sql_pool;
use crate::handlers::bet_only_regular_handler::bet_only_regular_handler;
use crate::balance_managers::bet_balance_manager::BetBalanceManager;
use crate::state::AppState;
use clap::{ Parser, Subcommand };
use crate::money::money::{Currency, Money};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Http {
        port: u16,
    },
    BetOnlyRegular {
        user_id: u64,
        amount: f64,
        currency_id: u16,
        currency_code: String,
    },
}

#[tokio::main]
async fn main() {
    get_sql_pool().await;
    let app_state = Arc::new(AppState::make_real());

    let cli = Cli::parse();

    dbg!(&cli);

    match cli.command {
        Command::Http { port } => {
            start_http_server(app_state).await;
        },
        Command::BetOnlyRegular {
            user_id,
            amount,
            currency_id,
            currency_code ,
        } => {
            let bet_balance_manager = BetBalanceManager::new(
                app_state,
                user_id,
                false,
                None,
            );

            bet_balance_manager.bet_only_regular(
                Money::new(
                    Currency::new(currency_id, currency_code),
                    amount,
                )
            ).await;
        }
    }
}

async fn start_http_server(app_state: Arc<AppState>) {
    let app = Router::new()
        .route(
            "/bet/only-regular",
            post(bet_only_regular_handler),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
