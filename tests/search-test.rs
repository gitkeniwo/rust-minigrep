use std::fs;

use minigrep::config;

#[test]
fn search_works() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], config::search(&query, &contents));
}

#[test]
fn search_works_on_file() {
    
    let query = "democracy";
    let contents = fs::read_to_string("padme.txt").expect("error");

    assert_eq!(vec!["So this is how democracy dies, with thunderous applause"], config::search(&query, &contents));
}