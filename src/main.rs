use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    
    println!("Searching for {}", config.query);
    println!("Looking in {}", config.file_path);
    // dbg!(args);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query : String,
    file_path : String
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config{query, file_path} 
// }

impl Config{
    fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config{query, file_path}
    }
}