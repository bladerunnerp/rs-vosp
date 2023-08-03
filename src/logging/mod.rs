use std::str::FromStr;

use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn init(level: &String) {
    SimpleLogger::new()
        .with_level(LevelFilter::from_str(&level).unwrap())
        .env() // override from env `RUST_LOG`
        .with_threads(true)
        .init()
        .unwrap()
}
