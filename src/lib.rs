use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let lines = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in lines {
        if config.ln {
            match line.number {
                0..=9 => println!("{}  | {}", line.number, line.content),
                10..=99 => println!("{} | {}", line.number, line.content),
                100..=999 => println!("{}| {}", line.number, line.content),
                _ => {
                    return Err(
                        format!("line number {}: unable to read any farther", line.number).into(),
                    );
                }
            }
        } else {
            println!("{}", line.content);
        }
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let mut lines = Vec::new();

    let mut number = 1;
    for line in contents.lines() {
        if line.contains(query) {
            lines.push(Line {
                number,
                content: line.to_string(),
            });
        }
        number += 1;
    }

    lines
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let query = query.to_lowercase();
    let mut lines = Vec::new();

    let mut number = 1;
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            lines.push(Line {
                number,
                content: line.to_string(),
            });
        }
        number += 1;
    }

    lines
}

#[derive(Debug, PartialEq)]
struct Line {
    number: i32,
    content: String,
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
    ln: bool,
}

// INTERLUDE: What's the difference between a parameter and an argument?
//
// arguments = the concrete values we pass when we call a function
// parameters = the definition of the stuff a function takes (the variables in a function's definition)
// > we provide arguments for parameters
//
//      parameter
//         |
//      vvvvvvv
// fn a(a: i128) {}
//
//   argument
//   |
//   v
// a(230)

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments\n\
                        Argument format: {query} {filename} (option)\n\
                        Options:\
                        \n    ln: include line numbers\n\
                        Example: `minigrep word list.txt ln`");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let mut ln = false;

        if args.len() > 3 {
            let option = &args[3];
            if option == "ln" {
                ln = true;
            } else {
                info("Invalid argument given for the third parameter");
            }
        }

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        // we will use `eprintln` for these messages too because we don't want them on the
        // standard output
        // after all, the standard error stream is for diagnostics too!
        if case_sensitive {
            info("CASE_SENSITIVE is set");
        } else {
            info("Set CASE_SENSITIVE for case-sensitivity");
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
            ln,
        })
    }
}

fn info(string: &str) {
    eprintln!("\x1b[38;5;11m{}\x1b[0m", string);
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![Line {
                number: 2,
                content: String::from("safe, fast, productive.")
            }],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                Line {
                    number: 1,
                    content: String::from("Rust:")
                },
                Line {
                    number: 4,
                    content: String::from("Trust me.")
                }
            ],
            search_case_insensitive(query, contents)
        );
    }
}
