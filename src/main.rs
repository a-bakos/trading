use std::string::ToString;

/**
 * TODO:
 * CLI position size calculator
 * - max position
 *
 */

enum PortfolioAsset {
    US500(f32),
    US100(f32),
    GOLD(f32),
    WTI(f32),
    UK100(f32),
    MSFT(f32),
}

impl PortfolioAsset {
    pub fn calculate_max_position_size(&self, account_size: f32) -> f32 {
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
                    multiplier: Some(50.0),
                }
            }
            PortfolioAsset::US100(price) => {
                Instrument {
                    name: "NASDAQ".to_string(),
                    ticker: "US100".to_string(),
                    asset_class: AssetClass::Index,
                    leverage: 20.0,
                    price: price.clone(),
                    multiplier: Some(20.0),
                }
            }
            PortfolioAsset::GOLD(price) => {
                Instrument {
                    name: "Gold".to_string(),
                    ticker: "GOLD".to_string(),
                    asset_class: AssetClass::Commodity,
                    leverage: 20.0,
                    price: price.clone(),
                    multiplier: Some(100.0),
                }
            }
            PortfolioAsset::WTI(price) => {
                Instrument {
                    name: "WTI Oil".to_string(),
                    ticker: "OIL.WTI".to_string(),
                    asset_class: AssetClass::Commodity,
                    leverage: 10.0,
                    price: price.clone(),
                    multiplier: Some(1000.0),
                }
            }
            PortfolioAsset::UK100(price) => {
                Instrument {
                    name: "FTSE100".to_string(),
                    ticker: "UK100".to_string(),
                    asset_class: AssetClass::Index,
                    leverage: 20.0,
                    price: price.clone(),
                    multiplier: Some(10.0),
                }
            }
            PortfolioAsset::MSFT(price) => {
                Instrument {
                    name: "Microsoft".to_string(),
                    ticker: "MSFT.US".to_string(),
                    asset_class: AssetClass::Stock,
                    leverage: 5.0,
                    price: price.clone(),
                    multiplier: None,
                }
            }
        }
    }
}

enum AssetClass {
    Index,
    Commodity,
    #[allow(dead_code)]
    Forex,
    Stock,
}

struct Instrument {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    ticker: String,
    #[allow(dead_code)]
    asset_class: AssetClass,
    leverage: f32,
    price: f32,
    multiplier: Option<f32>,
}

impl Instrument {
    #[allow(dead_code)]
    pub fn new(name: String, ticker: String, asset_class: AssetClass, leverage: f32, price: f32, multiplier: Option<f32>) -> Self {
        Self {
            name,
            ticker,
            asset_class,
            leverage,
            price,
            multiplier,
        }
    }

    pub fn calculate_max_position_size(&self, account_size: f32) -> f32 {
        self.get_levered_amount(account_size) / self.get_nominal_value_of_one_lot()
    }

    fn get_levered_amount(&self, account_size: f32) -> f32 {
        self.leverage * account_size
    }

    fn get_nominal_value_of_one_lot(&self) -> f32 {
        self.price * (match self.multiplier {
            Some(multiplier) => multiplier,
            None => 1.0
        })
    }
}

fn main() {
    let account_size: f32 = 1000.0;

    let max_pos_uk100 = PortfolioAsset::UK100(7349.3).calculate_max_position_size(account_size);
    println!("UK100: {}", max_pos_uk100);

    let max_pos_sp500 = PortfolioAsset::US500(4184.6).calculate_max_position_size(account_size);
    println!("US500: {:?}", max_pos_sp500);

    let max_pos_us100 = PortfolioAsset::US100(14411.67).calculate_max_position_size(account_size);
    println!("US100: {:?}", max_pos_us100);

    let max_pos_gold = PortfolioAsset::GOLD(1995.64).calculate_max_position_size(account_size);
    println!("GOLD: {:?}", max_pos_gold);

    let max_pos_oil = PortfolioAsset::WTI(82.56).calculate_max_position_size(account_size);
    println!("OIL: {:?}", max_pos_oil);

    let max_pos_msft = PortfolioAsset::MSFT(345.44).calculate_max_position_size(account_size);
    println!("MSFT: {:?}", max_pos_msft);
}
