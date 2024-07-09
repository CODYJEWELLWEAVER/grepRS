mod config;

use config::Config;
use std::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 1 {
        return Err(Box::from("At least one pattern must be given!"));
    }

    let run_config: Config = Config::new(args);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args() {
        let args = vec!();
        let result = run(args);
        assert!(!result.is_ok());
    }

    #[test]
    fn basic_run() {
        let args = vec!(String::from("dew"), String::from("../res/test/haiku.txt"));
        let result = run(args);
        assert!(result.is_ok());
    }
}