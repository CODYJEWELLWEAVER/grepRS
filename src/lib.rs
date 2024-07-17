pub mod config;
pub mod matcher;
pub mod source;
pub mod options;
pub mod output;

use config::Config;
use regex::{Regex, Matches};
use std::error::Error;
use std::io::{stderr, Write};

/// Runs search with command line arguments.
/// #### Param:
/// *   args - Vector of CL arguments.
/// See [Config] for more information about run configuration.
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 1 {
        return Err(Box::from("At least one pattern must be given!"));
    }

    let config: Config = Config::new(args)?;
    let regex: Regex = matcher::build_regex(&config.options)?;

    for mut source in config.sources {
        if let Err(msg) = source.read_content() {
            writeln!(
                &mut stderr(),
                "{} {}",
                "grepRS:",
                msg
            ).expect("Could not write to stderr.");

            continue;
        };

        let matches: Vec<Matches> = matcher::search_lines(
            &regex,
            &source.content
        );

        output::display(&config.options, &source, matches);
    }

    return Ok(());
}