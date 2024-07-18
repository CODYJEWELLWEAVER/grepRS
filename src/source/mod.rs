mod test;

use std::io::{stdin, Read};
use std::fs::File;

/// Holds path and content of source. A path of "-" represents stdin.
/// Used for representing content sources that will be searched
/// and pattern sources, when reading patterns from a file.
/// See Options::handle_pattern_file() for latter usage and Config::new()
/// for the former.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    pub path: String,
    pub content: String,
}

impl Source {
    /// Build new source from a path. Doesn't load the content
    /// to reduce memory impact.
    pub fn new(path: String) -> Source {
        Source {
            path,
            content: String::new(),
        }
    }

    /// Load source content.
    pub fn read_content (&mut self) -> Result<(), Box<std::io::Error>> {
        match self.path.as_str() {
            "-" => {
                stdin().read_to_string(&mut self.content)?;
            },
            _ => {
                let mut source_file = File::open(&self.path)?;
                source_file.read_to_string(&mut self.content)?;
            }
        }

        Ok(())
    }
}