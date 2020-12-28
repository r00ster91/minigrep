use std::{env, process};

use minigrep::Config;

// piping example: `minigrep \; src/lib.rs > output`

fn main() {
    let args: Vec<String> = env::args().collect();

    // for errors we will use the really handy macro called `eprintln` that will
    // not print to the standard output, but to the standard error stream
    // which means that if the user for example pipes the output to a file,
    // there won't be error messages in the file, but on the console!
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Operation error: {}", error);
        process::exit(1);
    }
}
