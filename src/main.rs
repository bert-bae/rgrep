mod rgrep;
mod step;
use crate::rgrep::{Cli, Rgrep};
use clap::Parser;
use colored::Colorize;
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("Beginning search...");
    let args = Cli::parse();
    let mut rgrep = Rgrep::new(args);
    match rgrep.search() {
        Ok(matches) => {
            for (k, v) in matches {
                println!("\n{}", k.purple());
                for line in v {
                    println!("{line}");
                }
            }
        }
        Err(e) => warn!("Issue searching files: {e}"),
    }
}
