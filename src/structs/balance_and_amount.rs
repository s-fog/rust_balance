use serde::{ Deserialize, Serialize };
use crate::structs::money::Money;

#[derive(Deserialize, Serialize, Debug)]
pub struct BalanceAndAmount {
    balance: u64,
    amount_money: Money,
}
