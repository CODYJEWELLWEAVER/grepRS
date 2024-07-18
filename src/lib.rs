pub mod config;
pub mod matcher;
pub mod source;
pub mod options;
pub mod output;

use config::Config;
use output::OutputBuffer;
use source::Source;
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
    let mut output_buffer = OutputBuffer::new();

    // used to detect when a source separator should be printed
    let last_source_idx = config.sources.len() - 1;
    let last_source: &Source = &config.sources[last_source_idx].clone();

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

        output_buffer.write_to_buffer(&config.options, &source, matches);

        // print source separator
        if source.path != last_source.path {
            output_buffer.write_separator();
        }
    }

    output_buffer.write_and_flush();

    return Ok(());
}