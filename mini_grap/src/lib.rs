use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_name: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        return Ok(Config { query: args[1].clone(), file_name: args[2].clone(), case_sensitive })
    }
}

/**
 * Search query from file content
 */
pub fn search_file(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "
        Rust:
        safe, fast, productive
        Pick, there.";
        assert_eq!(vec!["safe, fast, productive"], search(query, content));
    }

    #[test]
    fn insensitive_test() {
        let query = "to";
        let content = "
        Rust:
        To safe, fast, productive
        Pick, to there.";
        assert_eq!(vec!["To safe, fast, productive", "Pick, to there."], search_insensitive(query, content));
    }
}
