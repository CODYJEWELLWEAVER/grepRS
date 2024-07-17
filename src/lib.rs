mod config;
mod matcher;
mod source;
mod options;
mod output;

use config::Config;
use std::error::Error;
use source::Source;

/// Runs search with command line arguments.
/// #### Param:
/// *   args - Vector of CL arguments.
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 1 {
        return Err(Box::from("At least one pattern must be given!"));
    }

    let config: Config = Config::new(args)?;
    let regex = matcher::build_regex(&config)?;

    for path in &config.source_paths {
        let mut source = Source::new(path.to_string());
        source.load_content()?;

        let matches = matcher::search_lines(&regex, &source.content);

        output::display(&config, &source, matches);
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