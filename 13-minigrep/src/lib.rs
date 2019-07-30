//! # Minigrep
//! Small demo crate created following "the book" instructions, mainly from chaprets 12 and 13.
//! This performs a grep like action.
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let matches = search(&config.query, &contents);

    // if matches.len() == 0 {
    //     println!("No matches");
    // }

    for line in matches {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str > {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { query, filename })
    }
}

/// Search the given query on the contents, returning the lines that match.
///
/// # Example
///
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// assert_eq!(
///     vec!["safe, fast, productive."],
///     minigrep::search(query, contents)
/// );
/// ```
///
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
