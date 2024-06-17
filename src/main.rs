use std::{env, process};
use mygrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = mygrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}