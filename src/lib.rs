use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_contents(&config)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        dbg!(args);
        if args.len() < 3 {
            return Err("not enough arguments, please specify the query and the file_path");
        }

        let query = args[1].clone();
        let mut file_paths: Vec<String> = Vec::from(&args[2..]);

        // ignore case
        // command line argument --no-case takes precedence over env variable
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        if args.last().unwrap() == "--no-case" {
            ignore_case = true;
            file_paths.pop();
        }

        return Ok(Config {
            query,
            file_paths,
            ignore_case,
        });
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let query_lowercase = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lowercase) {
            result.push(line);
        }
    }

    result
}

fn read_contents(config: &Config) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();

    for file_path in &config.file_paths {
        let content = fs::read_to_string(file_path)?;
        contents.push_str(&content);
    }

    Ok(contents)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
