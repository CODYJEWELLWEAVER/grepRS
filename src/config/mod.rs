mod test;

use crate::options;
use crate::source;

use std::error::Error;
use regex::Regex;
use options::Options;
use source::Source;

/// Represents information about current search parameters.
/// #### Params:
/// *   patterns - patterns to search for.
/// *   files - files to search in.
/// *   options - optional run parameters, see Options.
#[derive(Clone)]
pub struct Config {
    pub sources: Vec<Source>,
    pub options: Options,
}

impl Config {
    /// Constructs new search configuration from command line arguments.
    pub fn new(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        let (source_args, option_args) = Self::preprocess_args(args)?;

        let mut options: Options = Options::default();

        if source_args.len() > 1 {
            options.file_prefix = true;
        }

        for arg in option_args {
            options.parse_option(arg)?;
        }

        let sources = source_args.into_iter().map(|path| {
            Source::new(path)
        }).collect();

        Ok(Config { sources, options })
    }

    /// Associates option arguments with their respective values.
    /// Returns a tuple with source/pattern args in the first vector
    /// and option args in the second.
    fn preprocess_args(args: Vec<String>) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let mut source_candidates = Vec::new();
        let mut options = Vec::new();
        // Checks if an option needs a value
        // e.g. '-e', so that the following value can be associated with it.
        let options_needs_value = Regex::new(r"^-[ef]$")?;

        // tracks if at least one pattern has been given explicitly
        let mut explicit_pattern: bool = false;

        let args_len = args.len();
        let mut i = 1; // excludes target arg
        while i < args_len {
            let arg: String = args[i].clone();

            if options_needs_value.is_match(&arg) {
                if i + 1 >= args_len {
                    return Err(Box::from(format!("Option: {}, requires a value!", arg)));
                }

                if arg == "-e" || arg == "--regexp" {
                    explicit_pattern = true;
                }

                // Associate next arg as the value.
                options.push(arg + "=" + &args[i+1]);
                i += 1;
            }
            else {
                source_candidates.push(arg);
            }

            i += 1;
        }

        if !explicit_pattern {
            // sets first source candidate as pattern if no
            // explicit pattern is set
            let pattern_arg: String = String::from("-e=") + &source_candidates.remove(0);
            options.push(pattern_arg);
        }

        Ok((source_candidates, options))
    }
}