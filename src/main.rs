use std::{env, fs, process};
fn main() {
    // input from args
    let args: Vec<String> = env::args().collect(); 
    
    let config = Config::build(&args).unwrap_or_else 
    (
        |err| 
        { 
            println!("Problem parsing error: {err}");
            process::exit(1);
        }
    );

    println!("Searcing for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("with text: \n{contents}");
}

struct Config{
    query: String,
    file_path: String,
}

// Construct func for Config
impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // build returns a Config or an Err-msg with static lifetime
        if args.len() < 3 {
            return Err("Not enough arugments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path })
    }
}
