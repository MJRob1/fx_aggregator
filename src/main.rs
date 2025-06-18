use fx_aggregator::FxMarketData;
use std::process;

fn main() {
    // read starting fx values from a file
    // manually set them below for now

    let lp = String::from("CITI");
    let instrument = String::from("EUR/USD");
    let price = 1.1552;

    // Get starting fx data for input values
    // just do the one lp above for now

    let mut fx_data = FxMarketData::new(lp, instrument, price);
    println!("initial fx_data is {fx_data:?}");

    // generate simulation market data with this initial fx data
    if let Err(e) = FxMarketData::generate(&mut fx_data) {
        println!("Error for now!: {e}");
        process::exit(1);
    }
}
