#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "INT")]
#[repr(u32)]
pub enum BalanceType {
    Regular = 1,
    RegularWager = 2,
    Bonus = 3,
    BonusWager = 4,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Balance {
    pub id: u64,
    r#type: BalanceType,
    user_bonus_id: Option<u64>,
}

impl Balance {
    pub fn new(
        id: u64,
        r#type: BalanceType,
        user_bonus_id: Option<u64>,
    ) -> Balance
    {
        Balance {
            id,
            r#type,
            user_bonus_id,
        }
    }
}
