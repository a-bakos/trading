use crate::{AssetClass, Currency, TradeDirection};

pub struct Instrument {
    #[allow(dead_code)]
    pub name: String,
    pub ticker: String,
    pub asset_class: AssetClass,
    pub leverage: f64,
    pub price: f64,
    pub currency: Currency,
    pub multiplier: Option<f64>,
    pub swap_rate_short: Option<f64>,
    pub swap_rate_long: Option<f64>,
}

impl Instrument {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        ticker: String,
        asset_class: AssetClass,
        leverage: f64,
        price: f64,
        currency: Currency,
        multiplier: Option<f64>,
        swap_rate_short: Option<f64>,
        swap_rate_long: Option<f64>,
    ) -> Self {
        Self {
            name,
            ticker,
            asset_class,
            leverage,
            price,
            currency,
            multiplier,
            swap_rate_short,
            swap_rate_long,
        }
    }

    pub fn calculate_max_position_size(&self, account_size: f64) -> f64 {
        self.get_levered_amount(account_size) / self.get_nominal_value_of_one_lot()
    }

    fn get_levered_amount(&self, account_size: f64) -> f64 {
        self.leverage * account_size
    }

    fn get_nominal_value_of_one_lot(&self) -> f64 {
        self.price * (match self.multiplier {
            Some(multiplier) => multiplier,
            None => 1.0
        })
    }

    // TODO
    // WIP: Calculation of daily swap points/overnight financing charge or credit
    fn calculate_swap_points(&self, trade_direction: TradeDirection, lots_number: f64) -> f64 {
        // FX:
        // 100_000 * lots number * daily swap rate % * conversion rate of account currency

        // Other instruments - such as indices
        // Instrument price * multiplier (used to calculate nominal value of 1 lot) * lots number * swap rate % * conversion rate of account currency

        let swap_rate = match trade_direction {
            TradeDirection::Long => self.swap_rate_long,
            TradeDirection::Short => self.swap_rate_short,
        };
        let swap_rate = if let Some(the_swap_rate) = swap_rate {
            the_swap_rate
        } else {
            1.0
        };
        let multiplier = if let Some(the_multiplier) = self.multiplier {
            the_multiplier
        } else {
            1.0
        };

        let conversion_rate = 1.0;

        let swap = self.price * multiplier * lots_number * swap_rate * conversion_rate;

        // Stocks CFD
        // transaction nominal * swap rate % * 1/360 * conversion rate of account currency

        0.0
    }
}
