use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
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
}

fn find_lines(mut args: Cli, pb: &ProgressBar) -> Result<Vec<String>, std::io::Error> {
    if args.case_sensitive == true {
        args.pattern = args.pattern.to_lowercase();
    }

    let file = File::open(&args.path).expect("File does not exist");
    let mut reader = BufReader::new(file);

    let mut found: Vec<String> = vec![];
    let mut line = String::new();
    let mut current_line = 1;
    while reader.read_line(&mut line)? != 0 {
        pb.tick();
        let matching_line: bool;
        if args.case_sensitive {
            matching_line = line.to_lowercase().contains(&args.pattern);
        } else {
            matching_line = line.contains(&args.pattern);
        }

        if matching_line {
            found.push(format!("[ln {current_line}] {}", line.replace("\n", "")));
        }
        current_line += 1;
        line.clear();
    }

    return Ok(found);
}

fn main() {
    let args = Cli::parse();
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(Duration::from_millis(200));
    progress_bar.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} cargo: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );
    let found = find_lines(args, &progress_bar);
    progress_bar.finish_and_clear();
    println!("{found:#?}")
}
