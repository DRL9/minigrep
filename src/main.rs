use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // - Why can it pass Vec<String>?
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("search for {}", config.query);
    println!("in file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        println!("error: {}", e);
        process::exit(1);
    }
}
