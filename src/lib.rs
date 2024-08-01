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
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // build returns a Config or an Err-msg with static lifetime
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // get ignore_case from both ENV_VALUE and args
        let ignore_case = env::var("IGNORE_CASE").map_or_else(
    |_| {
                // if env value doesn't exist, check args
                args.any(
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
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str)
-> Vec<&'a str> {
    // turn to lower case
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}