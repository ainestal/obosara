pub mod exchange_interactions;
pub mod influxdb;

fn say_hi() {
    println!("Hi !!!")
}

fn main() {
//    exchange_interactions::get_ticker();
//    influxdb::connect();
    say_hi()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
