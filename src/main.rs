use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    // #[arg(short = 'p', long = "pattern")]
    pattern: String,
    /// The path to the file to read
    // #[arg(short = 'f', long = "filepath")]
    path: std::path::PathBuf,
}

fn find_lines(args: Cli) -> Result<Vec<String>, std::io::Error> { 
    let file = File::open(&args.path).expect("File does not exist");
    let mut reader = BufReader::new(file);

    let mut found: Vec<String> = vec![];
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        if line.contains(&args.pattern) {
            found.push(line.clone());
        }
        line.clear();
    }
    
    return Ok(found)
}

fn main() {
    let args = Cli::parse();
    println!("{args:?}");

    let found = find_lines(args);
    println!("{found:?}")
}
