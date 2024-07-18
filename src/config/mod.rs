mod test;

use crate::options;
use crate::source;

use std::error::Error;
use regex::Regex;
use options::Options;
use source::Source;

/// Represents information about current search parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub sources: Vec<Source>,
    pub options: Options,
}

impl Config {
    /// Constructs new search configuration from command line arguments.
    pub fn new(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        let (mut source_args, option_args) = Self::preprocess_args(args)?;

        let mut options: Options = Options::default();

        if source_args.len() > 1 {
            options.file_prefix = true;
        }

        for arg in option_args {
            options.parse_option(arg)?;
        }

        if source_args.is_empty() {
            source_args.push(String::from("-"));
        }

        let sources = source_args.into_iter().map(|path| {
            Source::new(path)
        }).collect();

        Ok(Config { sources, options })
    }

    /// Associates option arguments with their respective values and separates
    /// source candidates from option arguments.
    /// Returns a tuple with source args in the first vector
    /// and option args in the second.
    fn preprocess_args(args: Vec<String>) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let mut source_candidates = Vec::new();
        let mut options = Vec::new();
        // Checks if an option needs a value
        // e.g. '-e', so that the following value can be associated with it.
        let option_needs_value = Regex::new(r"^-[ef]")?;

        // tracks if at least one pattern has been given explicitly
        let mut explicit_pattern: bool = false;

        let args_len = args.len();
        let mut i = 1; // excludes target arg
        while i < args_len {
            let arg: String = args[i].clone();

            if option_needs_value.is_match(&arg) {

                // Check if option arg needs to be associated with a value
                // e.g. args = {"./target", "-e", "dew"}
                if arg.len() == 2 {
                    if i + 1 >= args_len {
                        return Err(Box::from(format!("Option: {}, requires a value!", arg)));
                    }

                    // Associate next arg as the value.
                    options.push(String::from(&arg) + &args[i+1]);
                    i += 1;
                }
                else {
                    // if option already has a value associated with it
                    // e.g. args = {"./target", "-edew"}
                    options.push(String::from(&arg));
                }

                if &arg[..2] == "-e" || &arg == "--regexp" {
                    explicit_pattern = true;
                }
            }
            else if arg.len() > 1 && arg.starts_with("-") {
                // checks length to avoid detecting
                // stdin path "-" as an option
                options.push(arg);
            }
            else {
                source_candidates.push(arg);
            }

            i += 1;
        }

        if !explicit_pattern {
            // sets first source candidate as pattern if no
            // explicit pattern is set
            let pattern_arg: String = String::from("--regexp=") + &source_candidates.remove(0);
            options.push(pattern_arg);
        }

        Ok((source_candidates, options))
    }
}