use crate::source;
use crate::options;

use source::Source;
use options::Options;
use regex::Matches;
use std::iter::zip;
use std::io::{stdout, Write};

const BUFFER_SIZE: usize = 2048;

/// Contains methods for buffering and writing output.
pub struct OutputBuffer {
    pub buffer: String,
    pub out: Box<dyn Write>,
}

impl OutputBuffer {
    pub fn new() -> OutputBuffer {
        OutputBuffer {
            buffer: String::with_capacity(BUFFER_SIZE),
            out: Box::from(stdout()),
        }
    }

    /// Displays results of searching a [Source].
    pub fn write_to_buffer(
        &mut self,
        options: &Options,
        source: &Source,
        source_matches: Vec<Matches>
    ) {
        let source_lines = source.content.split("\n");
        let line_matches = zip(source_lines, source_matches);

        for (line, matches) in line_matches {
            let mut matches = matches.peekable();

            if matches.peek().is_some() && !options.invert_match {
                self.write_line(options, source, line);
            }
            else if matches.peek().is_none() && options.invert_match {
                self.write_line(options, source, line);
            }
        }
    }

    pub fn write_separator(&mut self) {
        self.buffer.push('\n');
    }

    pub fn write_and_flush(&mut self) {
        write!(
            self.out,
            "{}",
            self.buffer
        ).expect("grepRS: Could not write to stdout!");

        self.out.flush()
            .expect("grepRS: Could not flush buffer!");
    }

    // writes line to buffer
    fn write_line(&mut self, options: &Options, source: &Source, line: &str) {
        if options.file_prefix {
            let prefix = String::from(&source.path) + ":\t";
            self.buffer.push_str(prefix.as_str());
        }

        self.buffer.push_str(line);

        if !self.buffer.ends_with("\n") {
            self.buffer.push_str("\n");
        }
    }
}