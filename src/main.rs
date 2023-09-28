mod grrs;
mod step;
use crate::grrs::{ Cli, Grrs };
use clap::Parser;
use log::{ info, warn };

fn main() {
    env_logger::init();
    info!("Beginning search...");
    let args = Cli::parse();
    let mut grrs = Grrs::new(args);
    match grrs.search() {
        Ok(matches) => println!("{matches:#?}"),
        Err(e) => warn!("Issue searching files: {e}"),
    }
}
