use std::error::Error;
use std::{env, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = 
        "
        \nRust:
        \nsafe, fast, productive.
        \nPick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = 
        "
        \nRust:
        \nsafe, fast, productive.
        \nTrust me.
        ";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }


}

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Construct func for Config
impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // build returns a Config or an Err-msg with static lifetime
        if args.len() < 3 {
            return Err("Not enough arugments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // get ignore from both ENV_VALUE and args
        let ignore_case = env::var("IGNORE_CASE").map_or_else(
    |_| {
                // if env value doesn't exist, check args
                args.iter().any(
                    |arg| arg.to_lowercase() == "-i" || arg.to_lowercase() == "-ignore_case"
                )
            },
          |env_value| {
                // if env value exist, collect it's value
                env_value == "1" || env_value.to_lowercase() == "true"
          } 
        );

        Ok(Config{ query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // result storage
    let mut results = Vec::new();
    // iteration
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        println!("No target found");
    }
    
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str)
-> Vec<&'a str> {
    // turn to lower case
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        println!("No target found");
    }

    results
}