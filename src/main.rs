use std::{env, fs};
fn main() {
    // input from args
    let args: Vec<String> = env::args().collect(); 
    
    let config = parse_configs(&args);

    println!("Searcing for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("with text: \n{contents}");
}

struct Config{
    query: String,
    file_path: String,
}


fn parse_configs(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{ query, file_path }
}
