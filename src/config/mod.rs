mod test;

pub struct Config {
    patterns: Vec<String>,
    files: Vec<String>,
    options: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        let mut patterns: Vec<String> = Vec::new();
        let mut files: Vec<String> = Vec::new();
        let mut options: Vec<String> = Vec::new();

        for arg in args {
            if arg.starts_with("-") {
                options.push(arg);
            }
            else if patterns.len() > 0 {
                files.push(arg);
            }
            else {
                patterns.push(arg);
            }
        }

        return Config {
            patterns,
            files,
            options,
        };
    }
}