use rand::Rng;
use rand::prelude::*;

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
    fn update_to_next_value(&mut self) {
        // Calculate random price change up to a maximum of 5 pips
        // Randomly add or subtract this price change to calculate new fx rate
        println!("Initial fx buy price is {}", self.buy_prices[0]);
        let max_pip_change = 5.0; //5 pips - move to config later?
        let random_pip_change: f64 = rand::random_range(1.0..=max_pip_change);
        let random_price_change = random_pip_change / 10000.0; // Need to change for USD/JPY
        println!("The random price change is: {random_price_change}");
        // round this to 4 decimal places - seems this is the only way to do it in rust?  Need to change to 2 dec places for USD/JPY
        let rounded_price_change = (random_price_change * 10000.0).round() / 10000.0;
        println!("The rounded price change is: {rounded_price_change}");

        if rand::rng().random_bool(0.5) {
            self.buy_prices[0] += rounded_price_change;
            // println!("increase - new price is {new_price}");
        } else {
            self.buy_prices[0] -= rounded_price_change;
            //     println!("decrease - new price is {new_price}");
        };

        println!("price at end of loop is {}", self.buy_prices[0]);
    }

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
    pub fn generate(fx_data: &mut FxMarketData) -> Result<(), Box<dyn Error>> {
        fx_data.update_to_next_value();

        Ok(())
    }
}

fn new_price(price: f64, delta: f64) -> f64 {
    if rand::rng().random_bool(0.5) {
        let price = price + delta;
        println!("increase - new price is {price}");
        return price;
    } else {
        let price = price - delta;
        println!("decrease - new price is {price}");
        return price;
    }
}
