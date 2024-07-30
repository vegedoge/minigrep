use std::{env, process};

use minigrep::Config;
fn main() {
    // input from args
    let args: Vec<String> = env::args().collect(); 
    
    let config = Config::build(&args).unwrap_or_else 
    (
        |err| 
        { 
            eprintln!("Problem parsing error: {err}");
            process::exit(1);
        }
    );

    println!("Searcing for '{}' in {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }

}

