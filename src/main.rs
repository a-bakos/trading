const GBPUSD: f32 = 1.234;
const EURGBP: f32 = 1.234;

struct Account {
    balance: f32,
    base_currency: Currency,
}

impl Account {
    // TODO
    fn exchange_to_base(&self, asset_currency: Currency, asset_price: f32) -> f32 {
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

enum AssetClass {
    Index,
    Commodity,
    #[allow(dead_code)]
    Forex,
    Stock,
}

enum Currency {
    GBP,
    USD,
    EUR,
}

enum TradeDirection {
    Long,
    Short,
}

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

struct Instrument {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    ticker: String,
    #[allow(dead_code)]
    asset_class: AssetClass,
    leverage: f32,
    price: f32,
    currency: Currency,
    multiplier: Option<f32>,
    swap_rate_short: Option<f32>,
    swap_rate_long: Option<f32>,
}

impl Instrument {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        ticker: String,
        asset_class: AssetClass,
        leverage: f32,
        price: f32,
        currency: Currency,
        multiplier: Option<f32>,
        swap_rate_short: Option<f32>,
        swap_rate_long: Option<f32>,
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

    // TODO
    // WIP: Calculation of daily swap points/overnight financing charge or credit
    fn calculate_swap_points(&self, trade_direction: TradeDirection, lots_number: f32) -> f32 {
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

fn main() {
    let account = Account {
        balance: 1000.0,
        base_currency: Currency::GBP,
    };

    let max_pos_uk100 = PortfolioAsset::UK100(7349.3).calculate_max_position_size(account.balance);
    println!("UK100: {}", max_pos_uk100);

    let max_pos_sp500 = PortfolioAsset::US500(4184.6).calculate_max_position_size(account.balance);
    println!("US500: {:?}", max_pos_sp500);

    let max_pos_us100 = PortfolioAsset::US100(14411.67).calculate_max_position_size(account.balance);
    println!("US100: {:?}", max_pos_us100);

    let max_pos_gold = PortfolioAsset::GOLD(1995.64).calculate_max_position_size(account.balance);
    println!("GOLD: {:?}", max_pos_gold);

    let max_pos_oil = PortfolioAsset::WTI(82.56).calculate_max_position_size(account.balance);
    println!("OIL: {:?}", max_pos_oil);

    let max_pos_msft = PortfolioAsset::MSFT(345.44).calculate_max_position_size(account.balance);
    println!("MSFT: {:?}", max_pos_msft);
}
