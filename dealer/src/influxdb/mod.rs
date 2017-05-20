extern crate influent;

use self::influent::create_client;
use self::influent::client::{Client, Credentials};
use self::influent::measurement::{Measurement, Value};


pub fn connect() {

    // prepare client
    let credentials = Credentials {
        username: "gobwas",
        password: "xxx",
        database: "mydb"
    };
    let hosts = vec!["http://influxdb:8086"];
    let client = create_client(credentials, hosts);

    // prepare measurement
    let mut measurement = Measurement::new("key");
    measurement.add_field("some_field", Value::String("hello"));
    measurement.add_tag("some_region", "Moscow");

    match client.write_one(measurement, None) {
        Ok(v) => println!("Received: {:?}", v),
        _ => println!("Error")
    }
}