use std::{env, process};

use minigrep::Config;

// piping example: `minigrep \; src/lib.rs > output`

fn main() {
    let mut env_args = env::args();
    let program_name = env_args.next().unwrap(); // this is true at least most of the time
    let config = Config::new(env_args).unwrap_or_else(|error| {
        // for errors we will use this really handy macro called `eprintln` that will
        // not print to the standard output, but to the standard error stream
        // which means that if the user for example pipes the output to a file,
        // there won't be error messages in the file, but on the console!
        eprintln!(
            "Problem parsing arguments: {}\n\
             Argument format: {{query}} {{filename}} (optional: 'ln')\n\
             Example: `{} word list.txt ln`",
            error, program_name
        );
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Operation error: {}", error);
        process::exit(1);
    }
}
