use std::error::Error;
use std::fs;

pub struct Config{
    pub query: String,
    pub file_path: String,
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

        Ok(Config{ query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text: \n{contents}"); 

    Ok(())
}