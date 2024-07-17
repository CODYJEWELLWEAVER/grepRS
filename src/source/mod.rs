use std::io::{stdin, Read};
use std::error::Error;
use std::fs::File;

/// Holds path and content of source. An empty path represents stdin.
#[derive(Clone)]
pub struct Source {
    pub path: String,
    pub content: String,
}

impl Source {
    pub fn new(path: String) -> Source {
        Source {
            path,
            content: String::new(),
        }
    }

    /// Load content of source.
    pub fn load_content (&mut self) -> Result<(), Box<dyn Error>>{
        match self.path.as_str() {
            "" => {
                stdin().read_line(&mut self.content)?;
            },
            _ => {
                let mut source_file = File::open(&self.path)?;
                source_file.read_to_string(&mut self.content)?;
            }
        }

        Ok(())
    }
}