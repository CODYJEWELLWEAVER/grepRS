mod test;

use crate::options;

use std::error::Error;
use regex::Regex;
use options::Options;

/// Represents information about current search parameters.
/// #### Params:
/// *   patterns - patterns to search for.
/// *   files - files to search in.
/// *   options - optional run parameters, see Options.
pub struct Config {
    pub sources: Vec<String>,
    pub options: Options,
}

impl Config {
    /// Constructs new search configuration from command line arguments.
    pub fn new(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        let mut source_candidates: Vec<String> = Vec::new();
        let mut options: Options = Options::default();

        // associate values with their options
        let preprocessed_args = Self::preprocess_args(args)?;

        // filter options and source candidates
        for arg in preprocessed_args {
            if arg.starts_with("-") {
                options.parse_option(arg);
            }
            else {
                source_candidates.push(arg);
            }
        }

        // sets first non-option argument as pattern if no explicit patterns are set
        if options.patterns.len() == 0 {
            let mut pattern_str: &str = &source_candidates.remove(0);

            if pattern_str.starts_with("\"") && pattern_str.ends_with("\"") {
                pattern_str = &pattern_str[1..pattern_str.len() - 1]
            }

            for pattern in pattern_str.split("\n") {
                options.patterns.push(String::from(pattern));
            }
        }

        Ok(Config { sources: source_candidates, options })
    }

    /// Associates option arguments with their respective values.
    fn preprocess_args(args: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        let mut processed_args = Vec::new();
        // Checks if an option needs a value
        // e.g. '-e', so that the following value can be associated with it.
        let options_needs_value = Regex::new(r"^-[e]$")?;

        let args_len = args.len();
        let mut i = 0;
        while i < args_len {
            let arg: String = args[i].clone();
            if options_needs_value.is_match(&arg) {
                if i + 1 >= args_len {
                    return Err(Box::from(format!("Option: {}, requires a value!", arg)));
                }
                processed_args.push(arg + "=" + &args[i+1]); // Associate next arg as the value.
                i += 1;
            }
            else {
                processed_args.push(arg);
            }

            i += 1;
        }

        Ok(processed_args)
    }
}