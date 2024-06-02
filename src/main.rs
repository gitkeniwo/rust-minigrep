use std::process;
use minigrep;
use clap::Parser; // trait method minigrep::Config::parse(); can only be used if it is in scope

fn main() {
    let config = minigrep::Config::parse(); 
    // derive API will automatically parse the arguments and handle the errors

    if let Err(e) = minigrep::run(&config) {
        println!("Application Error {e}");
        process::exit(1);
    }
}
