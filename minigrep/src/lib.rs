use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // args[0] contains the name of the binary
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");

    let results = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}: {}", line.0, line.1);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> HashMap<u32, &'a str> {
    let mut results: HashMap<u32, &'a str> = HashMap::new();
    let mut line_number: u32 = 0;

    for line in contents.lines() {
        line_number += 1;
        if line.contains(query) {
            results.insert(line_number, line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> HashMap<u32, &'a str> {
    let mut results: HashMap<u32, &'a str> = HashMap::new();
    let mut line_number: u32 = 0;

    let query = query.to_lowercase();

    for line in contents.lines() {
        line_number += 1;
        if line.to_lowercase().contains(&query) {
            results.insert(line_number, line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let mut result = HashMap::new();
        result.insert(2, "safe, fast, productive.");

        assert_eq!(result, search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let mut result = HashMap::new();
        result.insert(1, "Rust:");
        result.insert(4, "Trust me.");

        assert_eq!(result, search_case_insensitive(query, contents));
    }
}
