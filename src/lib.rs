mod config;
mod matcher;
mod source;
mod options;
mod output;

use config::Config;
use regex::{Regex, Matches};
use std::error::Error;

/// Runs search with command line arguments.
/// #### Param:
/// *   args - Vector of CL arguments.
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 1 {
        return Err(Box::from("At least one pattern must be given!"));
    }

    let config: Config = Config::new(args)?;
    let regex: Regex = matcher::build_regex(&config)?;

    for mut source in config.sources {
        source.read_content()?;

        let matches: Vec<Matches> = matcher::search_lines(
            &regex,
            &source.content
        );

        output::display(&config.options, &source, matches);
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_run() {
        let args = vec!(String::from("./target"), String::from("dew"), String::from("res/test/haiku.txt"));
        let result = run(args);
        assert!(result.is_ok());
    }
}