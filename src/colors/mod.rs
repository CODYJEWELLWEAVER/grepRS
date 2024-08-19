mod test;

use std::env::var_os;

/// Defines the ANSI escape codes that should be used for output highlights.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Colors {
    /// Defines the formatting of matching text in selected lines. Defaults to bold yellow.
    pub selected_match: String,
    /// Defines the formatting of matching text in context lines. Defaults to bold yellow.
    pub context_match: String,
    /// Defines the formatting to be used for the entirety of selected lines.
    /// Defaults to the device's defaults.
    pub selected_line: String,
    /// Defines the formatting to be used for the entirety of context lines.
    /// Defaults to the device's defaults.
    pub context_line: String,
    /// Defines the formatting of file name prefixes. Defaults to green.
    pub file_name: String,
    /// Defines the formatting of line numbers. Defaults to red.
    pub line_number: String,
    /// Defines the formatting of byte offsets. Defaults to cyan.
    pub byte_offset: String,
    /// Defines the formatting of separators used in output. Defaults to magenta.
    pub separator: String,
}

impl Colors {
    /// Gets the colors to be used for a run. Looks for an environment variable
    /// named 'GREPRS_COLORS' and falls back to default values if no environment variable
    /// is found.
    pub fn get_colors() -> Colors {
        let colors_var = var_os("GREPRS_COLORS");

        return match colors_var {
            None => Self::default(),
            Some(colors) => {
                let colors = colors.into_string();

                if let Ok(color_str) = colors {
                    Self::parse_colors(color_str)
                } else {
                    Self::default()
                }
            }
        }
    }

    /// Builds default GREPRS Colors struct.
    fn default() -> Colors {
        Colors {
            selected_match: String::from("01;33"), // bold yellow
            context_match: String::from("01:33"), // bold yellow
            selected_line: String::from(""), // device default
            context_line: String::from(""), // device default
            file_name: String::from("32"), // green
            line_number: String::from("31"), // red
            byte_offset: String::from("36"), // cyan
            separator: String::from("35"), // magenta
        }
    }

    /// Constructs a Colors struct from a environment string variable.
    /// Uses defaults for any malformed or missing options.
    fn parse_colors(colors_str: String) -> Colors {
        let mut colors = Self::default();

        if !colors_str.is_ascii() {
            return colors;
        }

        let color_options = colors_str.split(":");

        for option in color_options.into_iter() {
            if option.is_ascii() && option.len() > 3 {
                let ansi_code = String::from(&option[3..]);

                match &option[..2] {
                    "mt" => {
                        colors.selected_match = ansi_code.clone();
                        colors.context_match = ansi_code;
                    },
                    "ms" => colors.selected_match = ansi_code,
                    "mc" => colors.context_match = ansi_code,
                    "sl" => colors.selected_line = ansi_code,
                    "cx" => colors.context_line = ansi_code,
                    "fn" => colors.file_name = ansi_code,
                    "ln" => colors.line_number = ansi_code,
                    "bn" => colors.byte_offset = ansi_code,
                    "se" => colors.separator = ansi_code,
                    _ => {
                        println!("GREPRS: Unknown color option: {}!", option[..2].to_string());
                    },
                }
            } else {
                println!("GREPRS: Invalid color option format!");
            }
        }

        colors
    }
}