mod test;

use std::io::{stdin, Read};
use std::fs::File;

/// Holds path and data of source. A path of "-" represents stdin.
/// Used for representing both content and pattern sources.
/// See `Options::handle_pattern_file` for latter usage and `Config::new`
/// for the former.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    /// Path to content or pattern source.
    pub path: String,
    /// Data in source path.
    pub data: String,
}

impl Source {
    /// Build new source from a path. `data` is initialized to an
    /// empty string and is the source data is read only when required.
    /// See [read_data](Source::read_data) for more details on how data is read.
    pub fn new(path: String) -> Source {
        Source {
            path,
            data: String::new(),
        }
    }

    /// Load source data.
    pub fn read_data (&mut self) -> Result<(), Box<std::io::Error>> {
        match self.path.as_str() {
            "-" => {
                stdin().read_to_string(&mut self.data)?;
            },
            _ => {
                let mut source_file = File::open(&self.path)?;
                source_file.read_to_string(&mut self.data)?;
            }
        }

        Ok(())
    }
}