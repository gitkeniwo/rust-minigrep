mod config;

use std::process;
use clap::Parser;

fn main() {
    let config = crate::config::Config::parse(); 
    // derive API will automatically parse the arguments and handle the errors

    if let Err(e) = crate::config::run(&config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}