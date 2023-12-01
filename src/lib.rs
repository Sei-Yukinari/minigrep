use std::error::Error;
use std::fs::File;
use std::io::Read;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename,case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_handles_case_sensitivity() {
        let args = vec![String::from(""), String::from("query"), String::from("filename")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.case_sensitive, std::env::var("CASE_INSENSITIVE").is_err());
    }

    #[test]
    fn search_returns_correct_lines() {
        let query = "safe";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        let results = search(query, contents);
        assert_eq!(results, vec!["safe, fast, productive."]);
    }

    #[test]
    fn search_case_insensitive_returns_correct_lines() {
        let query = "RUST";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        let results = search_case_insensitive(query, contents);
        assert_eq!(results, vec!["Rust:"]);
    }
}