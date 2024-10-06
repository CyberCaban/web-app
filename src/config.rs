use dotenv::dotenv;
use rocket::{
    data::{Limits, ToByteUnit},
    figment::value::magic::RelativePathBuf,
    Config,
};
use std::env;

pub fn from_env() -> Config {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());

    let config = Config {
        address: address.parse().unwrap(),
        port: port.parse().unwrap(),
        temp_dir: RelativePathBuf::from("tmp"),
        limits: Limits::default()
            .limit("file", 2.gigabytes())
            .limit("data-form", 2.gibibytes()),
        ..Config::default()
    };
    
    config
}
