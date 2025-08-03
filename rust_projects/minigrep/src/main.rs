use std::env; // access args
use std::process; // exit program without panicking

use minigrep::Config;

// cargo run hello content.txt
// cargo run hello non_existant.txt

/* 
export CASE_SENSITIVE=true
cargo run with content.txt
unset CASE_SENSITIVE
*/

// cargo run > output.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing args: {}", err);
        process::exit(1); // Requires 'use std::process'
    });

    println!("Search: {}", config.query);
    println!("Filename: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
