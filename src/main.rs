use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error processing args: {err}");
        process::exit(1);

    });
    
    // println!("Searching for {}", config.query);
    // println!("Looking in {}", config.file_path);
    // // dbg!(args);

    if let Err(e) = minigrep::run(config){
        println!("Application Error {e}");
        process::exit(1);
    }


}

