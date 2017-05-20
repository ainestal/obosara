extern crate coinnect;

use self::coinnect::coinnect::Coinnect;
use self::coinnect::exchange::{Exchange, ExchangeApi};
use self::coinnect::pair::Pair;


pub fn get_ticker() {
    // We create a BitstampApi by providing API key/secret/customer_id
    // You can give an empty str if you only use public methods
    let mut bitstamp_api: Box<ExchangeApi> = Coinnect::new(Exchange::Bitstamp,
                                                           "bs_api_key",
                                                           "bs_api_secret",
                                                           Some("bs_cust_id"));
    let ticker = bitstamp_api.ticker(Pair::BTC_USD);
    println!("Price : {}", ticker.unwrap().last_trade_price);
}
