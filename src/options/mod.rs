mod test;

use std::error::Error;
use std::env::var_os;

/// #### Options for a run of GrepRS.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Options {
    pub patterns: Vec<String>,
    pub color_output: bool,
    pub file_prefix: bool,
}

impl Options {
    /// Returns default Options.
    pub fn default() -> Options {
        Options {
            patterns: Vec::new(),
            color_output: Self::supports_color(),
            file_prefix: false,
        }
    }

    /// Parses option argument and applies option to current options struct.
    pub fn parse_option(&mut self, arg: String) -> Result<(), Box<dyn Error>> {
        let split_arg = arg.split_once("=");
        if let Some((option, value)) = split_arg {
            if option == "-e" || option == "--regexp" {
                let mut patterns = value;
                if patterns.starts_with("\"") && patterns.ends_with("\"") {
                    // remove enclosing double quotes
                    patterns = &patterns[1..patterns.len() - 1];
                }

                for pattern in patterns.split("\n") {
                    self.patterns.push(String::from(pattern));
                }
            }
            /* else if option == "-f" || option == "--file" {
                let mut pattern_buffer = String::new();
                let mut pattern_file = File::open(value)?;
                pattern_file.read_to_string(&mut pattern_buffer)?;

                for pattern in pattern_buffer.split("\n") {
                    self.patterns.push(String::from(pattern));
                }
            } */
            else {
                panic!("Invalid option: {}", option);
            }
        }
        else {
            panic!("Malformed option: {}", arg);
        }

        Ok(())
    }

    /// Check if running in a terminal that supports color
    /// ANSI color.
    // TODO: Add support for more environments.
    fn supports_color() -> bool {
        match var_os("COLORTERM") {
            Some(_) => {
                true
            },
            None => {
                false
            }
        }
    }
}