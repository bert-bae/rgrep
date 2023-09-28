mod step;
use crate::step::StepDir;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::fs::{metadata, read_dir, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    // #[arg(short = 'p', long = "pattern")]
    pattern: String,
    /// The path to the file to read
    // #[arg(short = 'f', long = "filepath")]
    path: std::path::PathBuf,
    #[arg(short = 'c', long = "case_sensitive", default_value_t = false)]
    case_sensitive: bool,
    #[arg(short = 'r', long = "recursive", default_value_t = false)]
    recursive: bool,
    #[arg(short = 'i', long = "ignore", default_value_t = String::new())]
    ignore: String,
}

fn ignore_file(path: &str, ignore: &str) -> bool {
    if ignore.is_empty() {
        return false;
    }

    let mut split = ignore.split(',');
    return split.any(|s| path.contains(s));
}

fn find_lines(
    path: std::path::PathBuf,
    args: &Cli,
    pb: &ProgressBar,
) -> Result<Vec<String>, std::io::Error> {
    let mut matches: Vec<String> = vec![];
    let file = File::open(&path).expect("File does not exist");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut current_line = 1;
    while reader.read_line(&mut line)? != 0 {
        pb.tick();
        let matching_line: bool;
        if args.case_sensitive {
            matching_line = line.to_lowercase().contains(&args.pattern.to_lowercase());
        } else {
            matching_line = line.contains(&args.pattern);
        }

        if matching_line {
            matches.push(format!(
                "[{} - ln {current_line}] {}",
                path.to_str().unwrap(),
                line.replace("\n", "")
            ));
        }
        current_line += 1;
        line.clear();
    }
    return Ok(matches);
}

fn create_directory_queue(
    root: std::path::PathBuf,
    ignore: Vec<String>,
) -> std::vec::IntoIter<PathBuf> {
    let step_dir = StepDir::new(root, ignore);
    step_dir.into_iter()
}

fn main() {
    env_logger::init();
    info!("Beginning search...");
    let args = Cli::parse();
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(Duration::from_millis(200));
    progress_bar.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} cargo: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );

    let file_path = args.path.clone();
    let queue = create_directory_queue(
        file_path,
        args.ignore.split(",").map(|s| String::from(s)).collect(),
    );

    let mut matching_lines: Vec<String> = vec![];
    for path in queue {
        match find_lines(path.clone(), &args, &progress_bar) {
            Ok(mut matches) => matching_lines.append(&mut matches),
            Err(e) => warn!("Encountered an error for {path:?}: {e}"),
        }
    }

    println!("{matching_lines:#?}");
    progress_bar.finish_and_clear();
}
