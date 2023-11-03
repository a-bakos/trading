use crate::{
    AssetClass, Currency,
    instrument::Instrument,
};

pub enum PortfolioAsset {
    US500(f64),
    US100(f64),
    GOLD(f64),
    WTI(f64),
    UK100(f64),
    MSFT(f64),
}

impl PortfolioAsset {
    pub fn calculate_max_position_size(&self, account_size: f64) -> f64 {
        self.details().calculate_max_position_size(account_size)
    }

    pub fn details(&self) -> Instrument {
        match self {
            PortfolioAsset::US500(price) => {
                Instrument {
                    name: "S&P 500".to_string(),
                    ticker: "US500".to_string(),
                    asset_class: AssetClass::Index,
                    leverage: 20.0,
                    price: price.clone(),
                    currency: Currency::USD,
                    multiplier: Some(50.0),
                    swap_rate_short: None,
                    swap_rate_long: Some(-0.025908),
                }
            }
            PortfolioAsset::US100(price) => {
                Instrument {
                    name: "NASDAQ".to_string(),
                    ticker: "US100".to_string(),
                    asset_class: AssetClass::Index,
                    leverage: 20.0,
                    price: price.clone(),
                    currency: Currency::USD,
                    multiplier: Some(20.0),
                    swap_rate_short: None,
                    swap_rate_long: Some(-0.025908),
                }
            }
            PortfolioAsset::GOLD(price) => {
                Instrument {
                    name: "Gold".to_string(),
                    ticker: "GOLD".to_string(),
                    asset_class: AssetClass::Commodity,
                    leverage: 20.0,
                    price: price.clone(),
                    currency: Currency::USD,
                    multiplier: Some(100.0),
                    swap_rate_short: None,
                    swap_rate_long: Some(-0.028686),
                }
            }
            PortfolioAsset::WTI(price) => {
                Instrument {
                    name: "WTI Oil".to_string(),
                    ticker: "OIL.WTI".to_string(),
                    asset_class: AssetClass::Commodity,
                    leverage: 10.0,
                    price: price.clone(),
                    currency: Currency::USD,
                    multiplier: Some(1000.0),
                    swap_rate_short: None,
                    swap_rate_long: Some(-0.027297),
                }
            }
            PortfolioAsset::UK100(price) => {
                Instrument {
                    name: "FTSE100".to_string(),
                    ticker: "UK100".to_string(),
                    asset_class: AssetClass::Index,
                    leverage: 20.0,
                    price: price.clone(),
                    currency: Currency::GBP,
                    multiplier: Some(10.0),
                    swap_rate_short: None,
                    swap_rate_long: Some(-0.026909),
                }
            }
            PortfolioAsset::MSFT(price) => {
                Instrument {
                    name: "Microsoft".to_string(),
                    ticker: "MSFT.US".to_string(),
                    asset_class: AssetClass::Stock,
                    leverage: 5.0,
                    price: price.clone(),
                    currency: Currency::USD,
                    multiplier: None,
                    swap_rate_short: None,
                    swap_rate_long: Some(-0.02722),
                }
            }
        }
    }
}