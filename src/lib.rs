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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // ignore 1st argument
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let mut file_paths: Vec<String> = args.collect();

        // ignore case
        // command line argument --no-case takes precedence over env variable
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        if file_paths.last().unwrap() == "--no-case" {
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

fn read_contents(config: &Config) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();

    for file_path in &config.file_paths {
        let content = fs::read_to_string(file_path)?;
        contents.push_str(&content);
        contents.push('\n');
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
