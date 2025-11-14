pub enum BalanceType {
    Regular,
    RegularWager,
    Bonus,
    BonusWager,
}

impl BalanceType {
    fn value(&self) -> u8
    {
        match self {
            BalanceType::Regular => 1,
            BalanceType::RegularWager => 2,
            BalanceType::Bonus => 3,
            BalanceType::BonusWager => 4,
        }
    }

    fn make_from_value(value: u8) -> Self
    {
        match value {
            1 => BalanceType::Regular,
            2 => BalanceType::RegularWager,
            3 => BalanceType::Bonus,
            4 => BalanceType::BonusWager,
        }
    }
}

pub struct Balance {
    id: u64,
    balance_type: BalanceType,
    user_bonus_id: Option(u64),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_money() {
        const AMOUNT: f64 = 11.22;
        const CURRENCY_ID: u16 = 12;

        let money = Money::new(CURRENCY_ID, AMOUNT);

        assert_eq!(CURRENCY_ID, money.currency_id);
        assert_eq!(AMOUNT, money.get_amount());
    }

    #[test]
    fn create_money_big_float() {
        let money = Money::new(9, 112.221545121);

        assert_eq!(112.22, money.get_amount());
    }
}
