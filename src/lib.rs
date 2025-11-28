use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub use_regex: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let use_regex = env::var("USE_REGEX").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            use_regex,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else if config.use_regex {
        search_with_regex(&config.query, &contents)?
    } else {
        search(&config.query, &contents)
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

pub fn search_with_regex<'a>(
    pattern: &str,
    contents: &'a str,
) -> Result<Vec<&'a str>, regex::Error> {
    let re = Regex::new(pattern)?;
    let mut results = Vec::new();

    for line in contents.lines() {
        if re.is_match(line) {
            results.push(line);
        }
    }

    Ok(results)
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

    #[test]
    fn empty_query() {
        let query = "";
        let contents = "Some text\nMore text";

        assert_eq!(vec!["Some text", "More text"], search(query, contents));
    }

    #[test]
    fn no_matches() {
        let query = "xyz";
        let contents = "Some text\nMore text";

        let expected: Vec<&str> = vec![];
        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn multiple_matches() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog.
The end.";

        assert_eq!(vec!["jumps over the lazy dog."], search(query, contents));
    }

    #[test]
    fn regex_search() {
        let pattern = r"\bnobody\b"; // Word boundary
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Somebody once told me.";

        let result = search_with_regex(pattern, contents).unwrap();
        assert_eq!(
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            result
        );
    }
}
