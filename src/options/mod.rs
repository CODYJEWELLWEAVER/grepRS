mod test;

/// #### Options for a run of GrepRS.
#[derive(PartialEq, Eq, Debug)]
pub struct Options {
    pub patterns: Vec<String>,
}

impl Options {
    /// Returns default Options.
    pub fn default() -> Options {
        Options {
            patterns: Vec::new(),
        }
    }

    /// Parses option argument and applies option to current options struct.
    pub fn parse_option(&mut self, arg: String) {
        if arg.starts_with("-e") || arg.starts_with("--regexp") {
            let split_arg = arg.split_once("=");
            if let Some((_l_half, r_half)) = split_arg {
                let mut patterns = r_half;
                if patterns.starts_with("\"") && patterns.ends_with("\"") {
                    // remove enclosing double quotes
                    patterns = &patterns[1..patterns.len() - 1];
                }

                for pattern in patterns.split("\n") {
                    self.patterns.push(String::from(pattern));
                }
            }
        }
        else {
            panic!("Invalid option: {}", arg);
        }
    }
}