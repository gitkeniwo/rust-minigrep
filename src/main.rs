use clap::{command, Parser};
// use std::env;
use std::fs;

#[derive(Parser)]
#[command(name = "minigrep")]
#[command(version = "0.1")]
#[command(about = "Does grep things", long_about = None)]
// #[command(name, version, about, long_about)] // or filling these info automatically on your cargo.toml
struct Cli {
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

fn main() {
    let cli = Cli::parse();

    // println!("File name: {:?}", cli.filename);
    println!("File name: {:?}", cli.filename);
    println!("Regex Mode: {:?}", cli.regexp);

    let msg = "Error reading the file.";
    let text = fs::read_to_string(cli.filename)
        .expect(msg);

    println!("Text: {:?}", text)
}
