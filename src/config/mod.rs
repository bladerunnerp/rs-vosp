use figment::{
    providers::{Env, Serialized},
    Figment,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub listen_addr: Ipv4Addr,
    pub port: u16,
    pub loglevel: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen_addr: Ipv4Addr::new(127, 0, 0, 1).to_owned(),
            port: 3000.to_owned(),
            loglevel: Some("info".to_string()),
        }
    }
}

impl Config {
    pub fn init() -> Config {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Env::prefixed("AUTH_"))
            .extract()
            .expect("Server configuration")
    }
    pub fn print(&self) -> Result<()> {
        let config = serde_json::to_string(&self)?;
        println!("{}", config);
        Ok(())
    }
}

// This will put the config into a global scope accessible with config::SETTINGS
lazy_static! {
    pub static ref SETTINGS: Config = Config::init();
}
