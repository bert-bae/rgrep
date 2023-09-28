use crate::step::StepDir;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// The pattern to search
    pattern: String,
    /// The root path to start the search from
    path: PathBuf,
    /// Case sensitive flag for file content search
    #[arg(short = 'c', long = "case_sensitive", default_value_t = false)]
    case_sensitive: bool,
    #[arg(short = 'r', long = "recursive", default_value_t = false)]
    recursive: bool,
    /// Ignores any files that contain this pattern
    #[arg(short = 'i', long = "ignore", default_value_t = String::new())]
    ignore: String,
}

pub struct Grrs {
    args: Cli,
    step_dir: StepDir,
}

impl Grrs {
    pub fn new(args: Cli) -> Self {
        let ignore = &args.ignore.to_owned();
        let ignored_file_patterns: Vec<String> =
            ignore.split(",").map(|s| String::from(s)).collect();
        Grrs {
            step_dir: StepDir::new(PathBuf::from(&args.path), ignored_file_patterns),
            args,
        }
    }

    pub fn search(&mut self) -> Result<Vec<String>, std::io::Error> {
        let mut matches: Vec<String> = vec![];
        let queue = self.queue();
        for path in queue {
            let file = File::open(&path).expect("File does not exist");
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            let mut current_line = 1;
            while reader.read_line(&mut line)? != 0 {
                // pb.tick();
                let matching_line: bool;
                if self.args.case_sensitive {
                    matching_line = line
                        .to_lowercase()
                        .contains(&self.args.pattern.to_lowercase());
                } else {
                    matching_line = line.contains(&self.args.pattern);
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

    fn queue(&mut self) -> std::vec::IntoIter<PathBuf> {
        let files = self.step_dir.clone().into_iter();
        files
    }
}
