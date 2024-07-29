use std::{env, fs};
fn main() {
    // input from args
    let args: Vec<String> = env::args().collect(); 
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searcing for {query} in {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("with text: \n{contents}");
}
