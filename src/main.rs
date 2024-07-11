use greprs::run;

extern crate greprs;

use std::env;
use std::process;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    match run(args) {
        Ok(_) => {
            process::exit(0);
        },
        Err(msg) => {
            writeln!(
                &mut stderr,
                "{} {}",
                "Error encountered while running GrepRS.",
                msg
            ).expect("Could not write to stderr.");

            process::exit(1);
        }
    };
}
