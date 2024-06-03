use clap::{command, Parser};
// use std::env;
use std::fs;
use std::error::Error;

// Arg Parsing using clap 4.x 's derive API 
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

    /// Search Query
    #[arg(index = 1)]
    query: String,

    /// Provide the path of the file
    #[arg(index = 2)]
    filename: String,
}


pub fn run(c: &Config) -> Result<(), Box<dyn Error>>{

    println!("File name: {:?}", &c.filename);
    println!("Query: {:?}", &c.query);
    println!("Regex Mode: {:?}", &c.regexp);

    // let msg = "Error reading the file";
    let contents = fs::read_to_string(&c.filename)?;
        // .expect(msg);
    // here, if you just use c.filename, 
    // error will occur saying "cannot move out of borrowed content"
    // so the correct way is to use reference: &c.filename

    // also, expect() / unwrap() already handles the error for you

    println!("---------------");

    for line in search(&c.query, &contents) {
        println!("{line}");
    }

    Ok(())
    // this single OK() here is just to return a Result type
    // if error occurs in the above, the error will automatically be returned
}

// Search Utility
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


