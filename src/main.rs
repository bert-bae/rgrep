mod grrs;
mod step;
use crate::grrs::{Cli, Grrs};
use clap::Parser;
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("Beginning search...");
    let args = Cli::parse();
    let mut grrs = Grrs::new(args);
    match grrs.search() {
        Ok(matches) => println!("{matches:#?}"),
        Err(e) => warn!("Issue searching files: {e}")
    }

    // let progress_bar = ProgressBar::new_spinner();
    // progress_bar.enable_steady_tick(Duration::from_millis(200));
    // progress_bar.set_style(
    //     ProgressStyle::with_template("{spinner:.dim.bold} cargo: {wide_msg}")
    //         .unwrap()
    //         .tick_chars("/|\\- "),
    // );

    // let file_path = args.path.clone();
    // let queue = create_directory_queue(
    //     file_path,
    //     args.ignore.split(",").map(|s| String::from(s)).collect(),
    // );
    //
    // let mut matching_lines: Vec<String> = vec![];
    // for path in queue {
    //     match find_lines(path.clone(), &args, &progress_bar) {
    //         Ok(mut matches) => matching_lines.append(&mut matches),
    //         Err(e) => warn!("Encountered an error for {path:?}: {e}"),
    //     }
    // }
    //
    // println!("{matching_lines:#?}");
    // progress_bar.finish_and_clear();
}
