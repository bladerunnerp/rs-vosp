use std::str::FromStr;

use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn init(level: Option<String>) {
    // if we passed in a level, us it
    let lvl = match level {
        Some(p) => LevelFilter::from_str(&p),
        _ => Ok(LevelFilter::Info), // or default to 'Info'
    };
    SimpleLogger::new()
        .with_level(lvl.unwrap())
        .env() // override from env `RUST_LOG`
        .with_threads(true)
        .init()
        .unwrap()
}
