pub mod money {
    use serde::{ Deserialize, Serialize };

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Currency {
        id: u16,
        code: String,
    }

    impl Currency {
        pub fn new(
            id: u16,
            code: String,
        ) -> Self
        {
            Self {
                id,
                code,
            }
        }

        pub fn get_id(&self) -> &u16 {
            &self.id
        }

        pub fn get_code(&self) -> &String {
            &self.code
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Money {
        currency: Currency,
        raw_amount: f64,
    }

    impl Money {
        pub fn new(
            currency: Currency,
            raw_amount: f64,
        ) -> Self
        {
            Self {
                currency,
                raw_amount,
            }
        }

        pub fn get_amount(&self) -> f64 {
            (self.raw_amount * 100.0).round() / 100.0
        }

        pub fn get_currency(&self) -> &Currency {
            &self.currency
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_money() {
            let amount: f64 = 11.22;
            let currency_id: u16 = 12;
            let currency_code: String = String::from("RUB");

            let currency = Currency::new(currency_id, currency_code);

            let money = Money::new(currency, amount);

            assert_eq!(currency_id, money.currency.id);
            assert_eq!(amount, money.get_amount());
        }

        #[test]
        fn create_money_big_float() {
            let amount: f64 = 112.221545121;
            let currency_id: u16 = 12;
            let currency_code: String = String::from("RUB");

            let currency = Currency::new(currency_id, currency_code);

            let money = Money::new(currency, amount);

            assert_eq!(112.22, money.get_amount());
        }
    }

}