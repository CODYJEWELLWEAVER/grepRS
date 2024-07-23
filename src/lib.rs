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
//! I fully plan to support both in the future. I am still in the early stages of development with only the most basic
//! functionality implemented. Searching basic patterns in text files is fully functional and tested. Other functionality
//! has not been thoroughly tested at this point but should largely work. Color output, advanced output
//! options, and support for searching additional formats is on my radar next. Color output and thorough testing
//! of complex usage are next on my todo list.
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

use config::Config;
use output::OutputBuffer;
use source::Source;
use regex::{Regex, Matches};
use std::error::Error;
use std::io::{stderr, Write};

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

    // used to detect when a source separator should be printed
    let last_source_idx = config.sources.len() - 1;
    let last_source: &Source = &config.sources[last_source_idx].clone();

    for mut source in config.sources {
        if let Err(msg) = source.read_data() {
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
            &source.data
        );

        output_buffer.append_source_results(&config.options, &source, matches);

        // print source separator
        if source.path != last_source.path {
            output_buffer.append_separator();
        }
    }

    output_buffer.write_and_flush();

    return Ok(());
}