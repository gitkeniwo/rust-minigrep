use clap::{command, Parser};
// use std::env;
use std::fs;
use std::error::Error;

/// Arg Parsing using clap 4.x 's derive API
/// 
/// # Examples
/// 
#[derive(Parser)]
#[command(name = "minigrep")]
#[command(version = "0.1")]
#[command(about = "Does grep things", long_about = None)]
// #[command(name, version, about, long_about)] // or filling these info automatically based on your cargo.toml
pub struct Config {

    /// Enable regex pattern matching
    #[arg(
        short = 'e',
        long = "regexp",
        required = false,
        default_value_t = false
    )]
    regexp: bool,

    /// Case Insensitive Search
    #[arg(
        short = 'c',
        long = "ignore-case", 
        required = false, 
        default_value_t = true,
        )]
    ignore_case: bool,

    /// Search Query
    #[arg(index = 1)]
    query: String,

    /// Provide the path of the file
    #[arg(index = 2)]
    filename: String,

}

/// Execute the search process
pub fn run(c: &Config) -> Result<(), Box<dyn Error>>{

    println!("File name: {:?}", &c.filename);
    println!("Query: {:?}", &c.query);
    println!("Case Insensitive: {:?}", &c.ignore_case);
    println!("Regex Mode: {:?}", &c.regexp);

    // let msg = "Error reading the file";
    let contents = fs::read_to_string(&c.filename)?; // ? will return the error if it occurs
        // .expect(msg);
    // here, if you just use c.filename, 
    // error will occur saying "cannot move out of borrowed content"
    // so the correct way is to use reference: &c.filename

    // also, expect() / unwrap() already handles the error for you

    println!("---------------");

    let results = if c.ignore_case {
        search_case_insensitive(&c.query, &contents)
    } else {
        search(&c.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
    // this single OK() here is just to return a Result type
    // if error occurs in the above, the error will automatically be returned
}

/// Search Utility
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Insensitive Search Utility
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase(); // String, instead of &str
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}



// Unit Tests
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
