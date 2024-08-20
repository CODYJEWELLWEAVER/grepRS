#![warn(missing_docs)]
//! ## DESCRIPTION
//!
//! An implementation of _grep_ in Rust. This project was inspired by the mini project in "The Rust Book".
//! I decided I wanted to expand on the idea and try to fully write _grep_ using Rust as a way to
//! increase my knowledge of both. All credit for [_grep_](https://www.gnu.org/software/grep/manual/grep.html)
//! goes to Ken Thompson & AT&T Bell Laboratories. This project is licensed under GPLv3 which you can find
//! [here](https://www.gnu.org/licenses/gpl-3.0.en.html#license-text).
//!
//! I have made it a point of emphasis to keep the usage of grepRS identical to the usage of _grep_ so far.
//! My hope is to allow for grepRS to be used without having to learn anything new.
//!
//! As of right now only unix is fully supported and this project may not completely run on macOS or Windows. However,
//! I fully plan to support both in the future.
//!
//! Note: The rust lang regex package does not support look-around. I plan on implementing look-around eventually, but
//! this will be far in the future. Currently, look-around patterns will not work.
//!
//! ## BASIC USAGE
//!
//! ### **Basic**
//!
//! ```text
//! greprs [options...] pattern [sources...]
//! ```
//!
//! Currently, a source can be either text file or stdin.
//!
//! There are no restrictions on where options must be given in the command.
//!
//! ```text
//! greprs --ignore-case pattern source # valid
//! greprs pattern source --ignore-case # also valid
//! ```
//!
//! If only one non-option argument is given it is interpreted as a pattern
//! and stdin will be used as the source for content to search in.


//! ## CONTACT
//!
//! Found a bug? Hit me up here:
//! *   Email: cody.weaver@colorado.edu
//! *   GitHub: [CODYJEWELLWEAVER](https://github.com/CODYJEWELLWEAVER)

/// Encapsulates [options](options::Options) and [sources](source::Source)
/// for a run of grepRS.
pub mod config;
/// Contains methods for finding matches in a [source](Source).
pub mod matcher;
/// Holds information about a content source that will be searched for matches or patterns.
pub mod source;
/// Execution settings and associated logic for parsing them from command line arguments.
pub mod options;
/// Handles buffering and writing output.
pub mod output;
/// Handles colors used for output highlights
pub mod colors;

use config::Config;
use output::OutputBuffer;
use regex::{Regex, Matches};
use std::error::Error;
use std::io::{stderr, Write, ErrorKind};

/// Runs grepRS with command line arguments.
/// #### Param:
/// *   args - Vector of CL arguments.
/// See [Config] for more information about run configuration.
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err(Box::from("At least one pattern must be given!"));
    }

    let config: Config = Config::new(args)?;
    let regex: Regex = matcher::build_regex(&config.options)?;
    let mut output_buffer = OutputBuffer::default();

    let options = &config.options;

    for mut source in config.sources {
        if let Err(io_err) = source.read_data() {
            if !options.no_messages {
                print_io_err_msg(io_err, &source.path);
            }

            continue;
        };

        if options.count_lines {
            // COUNT MATCHING LINES
            let source_counts: usize = matcher::count_matching_lines(
                &regex,
                &source.data,
                options.invert_match
            );

            output_buffer.append_source_counts(options, &source, source_counts);
        }
        else {
            // SEARCH FOR MATCHES IN LINES
            let matches: Vec<Matches> = matcher::search_lines(
                &regex,
                &source.data
            );

            output_buffer.append_source_matches(options, &source, matches);
        }
    }

    if !options.silent {
        output_buffer.write_and_flush();
    }

    return Ok(());
}

/// Prints a message to stderr explaining an IO error.
fn print_io_err_msg(io_err: Box<std::io::Error>, path: &str) {
    let err_msg = match io_err.kind() {
        ErrorKind::NotFound => {
            format!("{} not found!", path)
        },
        ErrorKind::PermissionDenied => {
            format!("Insufficient permissions to read from {}", path)
        },
        _ => {
            io_err.to_string()
        }
    };

    writeln!(
        stderr(),
        "{} {}",
        "grepRS:",
        err_msg,
    ).expect("grepRS: could not write to stderr!");
}