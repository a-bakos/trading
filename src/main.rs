mod account;
mod portfolio_asset;
mod instrument;

use crate::{
    account::Account,
    portfolio_asset::PortfolioAsset,
};

const GBPUSD: f32 = 1.234;
const EURGBP: f32 = 1.234;

pub enum AssetClass {
    Index,
    Commodity,
    #[allow(dead_code)]
    Forex,
    Stock,
}

pub enum Currency {
    GBP,
    USD,
    EUR,
}

pub enum TradeDirection {
    Long,
    Short,
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
