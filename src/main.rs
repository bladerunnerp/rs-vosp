// #![allow(unused)]
mod config;
mod logging;
use crate::{cli::Cli, config::SETTINGS};
mod cli;
use log::info;

#[tokio::main]
async fn main() {
    let cli = Cli::init();
    let loglevel = &SETTINGS.loglevel;
    logging::init(loglevel);

    info!("Initialized...");

    if cli.print_config == true {
        SETTINGS.print()
    }
}
