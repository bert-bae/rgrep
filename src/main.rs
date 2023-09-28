use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::fs::{metadata, read_dir, File};
use std::io::prelude::*;
use std::io::BufReader;
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
    ignore: String
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
    if ignore_file(path.to_str().unwrap(), &args.ignore) {
        return Ok(matches);
    }


    let file = File::open(&path).expect("File does not exist");
    let md = metadata(&path).unwrap();

    if md.is_dir() {
        let directory = read_dir(&args.path).unwrap();
        for f in directory {
            let f = f.unwrap();
            let mut found = find_lines(f.path(), args, pb)?;
            matches.append(&mut found);
        }
    } else {
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
    }

    return Ok(matches);
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

    match find_lines(args.path.clone(), &args, &progress_bar) {
        Ok(matches) => {
            info!("Search complete. Found {} matching lines.", &matches.len());

            for line in matches {
                println!("{line}");
            }
        }
        Err(e) => warn!("Error searching file: {e}"),
    };
    progress_bar.finish_and_clear();
}
