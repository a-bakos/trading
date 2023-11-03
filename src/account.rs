use crate::Currency;

pub struct Account {
    pub balance: f32,
    pub base_currency: Currency,
}

impl Account {
    // TODO
    pub fn exchange_to_base(&self, asset_currency: Currency, asset_price: f32) -> f32 {
        match asset_currency {
            Currency::GBP => {
                match self.base_currency {
                    Currency::GBP => return asset_price,
                    Currency::USD => {}
                    Currency::EUR => {}
                }
            }
            Currency::USD => {
                match self.base_currency {
                    Currency::GBP => {}
                    Currency::USD => return asset_price,
                    Currency::EUR => {}
                }
            }
            Currency::EUR => {
                match self.base_currency {
                    Currency::GBP => {}
                    Currency::USD => {}
                    Currency::EUR => return asset_price,
                }
            }
        }

        0.0
    }
}
