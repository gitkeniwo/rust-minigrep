use std::process;
use clap::Parser;

pub mod config;

fn main() {
    let config = config::Config::parse(); 
    // derive API will automatically parse the arguments and handle the errors

    if let Err(e) = config::run(&config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}