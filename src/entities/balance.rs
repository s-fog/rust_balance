use serde::{ Serialize, Deserialize };

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "INT")]
#[repr(u8)]
pub enum BalanceType {
    Regular = 1,
    RegularWager = 2,
    Bonus = 3,
    BonusWager = 4,
}

impl BalanceType {
    pub fn get_value(&self) -> u8 {
        match self {
            BalanceType::Regular => 1,
            BalanceType::RegularWager => 2,
            BalanceType::Bonus => 3,
            BalanceType::BonusWager => 4,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Balance {
    id: u64,
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

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_type(&self) -> BalanceType {
        self.r#type
    }

    pub fn get_user_bonus_id(&self) -> Option<u64> {
        self.user_bonus_id
    }
}
