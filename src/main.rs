extern crate greprs;

use greprs::run;
use std::env;
use std::process;
use std::io::{Write, stderr};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = stderr();

    match run(args) {
        Ok(_) => {
            process::exit(0);
        },
        Err(msg) => {
            writeln!(
                &mut stderr,
                "{} {}",
                "Error encountered while running grepRS.",
                msg
            ).expect("Could not write to stderr.");

            process::exit(1);
        }
    };
}
