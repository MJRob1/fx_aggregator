use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct FxMarketData {
    liquidity_provider: String,
    instrument: String,
    buy_prices: [f64; 3],
    sell_prices: [f64; 3],
    volumes: [i32; 3],
    timestamp: u128,
}

impl FxMarketData {
    pub fn new(lp: String, instrument: String, price: f64) -> FxMarketData {
        let liquidity_provider = lp;
        let instrument = instrument;
        let buy_prices = [price, price - 0.0003, price - 0.0005];
        let sell_prices = [price + 0.0006, price + 0.0004, price + 0.0002];
        let volumes = [1, 3, 5];
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        FxMarketData {
            liquidity_provider,
            instrument,
            buy_prices,
            sell_prices,
            volumes,
            timestamp,
        }
    }
    pub fn generate(fx_data: FxMarketData) -> Result<(), Box<dyn Error>> {
        println!("Initial fx buy price is {}", fx_data.buy_prices[0]);
        Ok(())
    }
}
