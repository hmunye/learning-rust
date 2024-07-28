use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Failed to parse argmuments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for \"{}\" in file: {}",
        config.query, config.file_path
    );

    if let Err(err) = minigrep::run(config) {
        eprintln!("Program error occured: {err}");
        process::exit(1);
    }
}
