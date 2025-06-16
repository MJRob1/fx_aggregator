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

        // generate random number from previous price to previous price + 2 pips - make 2 pips a config option later
        let increment: f64 =
            rand::random_range(fx_data.buy_prices[0]..=fx_data.buy_prices[0] + 0.0002)
                - fx_data.buy_prices[0];
        println!("The increment is: {increment}");

        // round this to 4 decimal places - seems this is the only way to do it in rust?
        let delta = (increment * 10000.0).round() / 10000.0;
        println!("The delta is: {delta}");

        let direction = rand::rng().random_bool(0.5);
        //println!("True or false: {}", rng.random::<bool>());
        println!("direction is {direction}");

        if direction {
            let new_price = fx_data.buy_prices[0] + delta;
            println!("new rice is {new_price}");
        } else {
            let new_price = fx_data.buy_prices[0] - delta;
            println!("new rice is {new_price}");
        }

        Ok(())
    }
}
