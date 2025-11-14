#![allow(dead_code, unused)]

use crate::database_connection::database_connection::Mysql;

mod env_load;
mod database_connection;
mod cache;
mod structs;
mod redis_client;

fn main() {
    Mysql::get_pool()
}
