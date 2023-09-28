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
    case_insensitive: bool,
    /// Recursively check all directories within the root path
    #[arg(short = 'r', long = "recursive", default_value_t = false)]
    recursive: bool,
    /// Ignores any files that contain this pattern
    #[arg(short = 'i', long = "ignore", default_value_t = String::new())]
    ignore: String,
}

pub struct Rgrep {
    args: Cli,
    step_dir: StepDir,
}

impl Rgrep {
    pub fn new(args: Cli) -> Self {
        let ignore = &args.ignore.to_owned();
        let ignored_file_patterns: Vec<String> =
            ignore.split(",").map(|s| String::from(s)).collect();
        Rgrep {
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
            let mut buf = vec![];
            let mut current_line = 1;

            while let Ok(_) = reader.read_until(b'\n', &mut buf) {
                if buf.is_empty() || std::str::from_utf8(&buf).is_err() {
                    break;
                }

                let line = String::from_utf8_lossy(&buf);
                let matching_line: bool;
                if self.args.case_insensitive {
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
                buf.clear();
            }
        }
        return Ok(matches);
    }

    fn queue(&mut self) -> std::vec::IntoIter<PathBuf> {
        if self.args.recursive {
            let files = self.step_dir.clone().into_iter();
            files
        } else {
            vec![self.args.path.clone()].into_iter()
        }
    }
}
