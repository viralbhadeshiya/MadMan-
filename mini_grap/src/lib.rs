use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_name: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let mut args = env::args();

        // Discard first element
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query value is missing"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename is missing"),
        };
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        return Ok(Config { query, file_name, case_sensitive })
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
    content.lines().filter(|line| line.contains(query)).collect()
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
