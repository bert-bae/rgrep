use std::fs::{metadata, read_dir};

use std::iter::IntoIterator;
use std::path::PathBuf;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub struct StepDir {
    root: PathBuf,
    files: Vec<PathBuf>,
    ignore: Vec<String>,
}

impl IntoIterator for StepDir {
    type Item = PathBuf;
    type IntoIter = IntoIter<PathBuf>;

    fn into_iter(mut self) -> IntoIter<PathBuf> {
        let _ = self.step_dirs(self.root.clone()).unwrap();
        self.files.into_iter()
    }
}

impl StepDir {
    pub fn new(root: PathBuf, ignore: Vec<String>) -> Self {
        StepDir {
            root,
            ignore,
            files: vec![]
        }
    }
   
    fn step_dirs(&mut self, path: PathBuf) -> Result<bool, std::io::Error> {
        let path_string = &path.to_str().to_owned().unwrap();
        if self.ignore.iter().any(|s| path_string.contains(s)) {
            return Ok(true);
        }

        let md = metadata(&path).unwrap();
        if md.is_dir() {
            let files = read_dir(path).unwrap();
            for f in files {
                let f = f.unwrap();
                self.step_dirs(f.path())?;
            }
        } else {
            self.files.push(path);
        }

        Ok(true)
    }
}
