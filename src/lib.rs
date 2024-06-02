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

    /// Provide the path of the file
    #[arg(index = 1)]
    filename: String,
}


pub fn run(c: &Config) -> Result<(), Box<dyn Error>>{

    println!("File name: {:?}", &c.filename);
    println!("Regex Mode: {:?}", &c.regexp);

    let msg = "Error reading the file";
    let text = fs::read_to_string(&c.filename)
        .expect(msg);
    // here, if you just use c.filename, 
    // error will occur saying "cannot move out of borrowed content"
    // so the correct way is to use reference: &c.filename

    println!("Text: {text}");

    Ok(())
}