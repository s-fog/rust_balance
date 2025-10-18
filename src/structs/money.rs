pub struct Money {
    pub currency_id: u16,
    raw_amount: f64,
}

impl Money {
    pub fn new(currency_id: u16, amount: f64) -> Money {
        Money {
            currency_id,
            raw_amount: amount,
        }
    }

    pub fn get_amount(&self) -> f64 {
        (self.raw_amount * 100.0).round() / 100.0
    }
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
