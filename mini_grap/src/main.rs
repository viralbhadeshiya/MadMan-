use std::process;
use mini_grap::Config;

fn main() {
    // Parsing arguments form env
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsin args: {}", err);
        process::exit(1);
    });
    println!("Seraching for {} in file {}", config.query, config.file_name);

    // Search from file data
    if let Err(e) = mini_grap::search_file(&config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
