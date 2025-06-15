use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let lp = String::from("CITI");
    let instrument = String::from("EUR/USD");
    let price = 1.1552;

    let fx_data = FxMarketData::new(lp, instrument, price);
    println!(
        "FX data is {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        fx_data.liquidity_provider,
        fx_data.instrument,
        fx_data.timestamp,
        fx_data.buy_prices[0],
        fx_data.buy_prices[1],
        fx_data.buy_prices[2],
        fx_data.sell_prices[0],
        fx_data.sell_prices[1],
        fx_data.sell_prices[2],
        fx_data.volumes[0],
        fx_data.volumes[1],
        fx_data.volumes[2]
    );
}

struct FxMarketData {
    liquidity_provider: String,
    instrument: String,
    buy_prices: [f64; 3],
    sell_prices: [f64; 3],
    volumes: [i32; 3],
    timestamp: u128,
}

impl FxMarketData {
    fn new(lp: String, instrument: String, price: f64) -> FxMarketData {
        let liquidity_provider = lp;
        let instrument = instrument;
        let buy_prices = [price, price - 0.03, price - 0.05];
        let sell_prices = [price + 0.06, price + 0.04, price + 0.02];
        let volumes = [1, 3, 5];
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        FxMarketData {
            liquidity_provider: liquidity_provider,
            instrument: instrument,
            buy_prices: buy_prices,
            sell_prices: sell_prices,
            volumes: volumes,
            timestamp: timestamp,
        }
    }
}
