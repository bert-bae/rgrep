mod rgrep;
mod step;
use crate::rgrep::{Cli, Rgrep};
use clap::Parser;
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("Beginning search...");
    let args = Cli::parse();
    let mut rgrep = Rgrep::new(args);
    match rgrep.search() {
        Ok(matches) => {
            for line in matches {
                println!("{line}");
            }
        },
        Err(e) => warn!("Issue searching files: {e}"),
    }
}
