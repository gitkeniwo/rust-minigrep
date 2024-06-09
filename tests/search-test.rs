use std::fs;

use minigrep_util::config;

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



#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], config::search(query, contents));
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
        config::search_case_insensitive(query, contents)
    );
}