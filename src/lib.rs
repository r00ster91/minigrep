use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let lines = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    let capacity = lines
        .iter()
        .map(|line| {
            if config.ln {
                line.number_len() + line.separator().len() + line.content.len() + 1
            } else {
                line.content.len() + 1
            }
        })
        .sum();
    let mut output = String::with_capacity(capacity);

    for line in lines {
        if config.ln {
            output.push_str(&line.number.to_string());
            output.push_str(line.separator());
            output.push_str(&line.content);
        } else {
            output.push_str(&line.content);
        }
        output.push('\n');
    }

    print!("{}", output);

    // println!(
    //     "capacity: {}, len: {}, calculated capacity: {}",
    //     output.capacity(),
    //     output.len(),
    //     capacity
    // );

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    contents
        .lines()
        .zip(1..) // One-based line numbers
        .filter(|(content, _)| content.contains(query))
        .map(|(content, number)| Line {
            number,
            content: content.to_string(),
        })
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    contents
        .lines()
        .zip(1..) // One-based line numbers
        .filter(|(content, _)| content.to_lowercase().contains(&query.to_lowercase()))
        .map(|(content, number)| Line {
            number,
            content: content.to_string(),
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct Line {
    number: usize,
    content: String,
}

impl Line {
    fn number_len(&self) -> usize {
        match self.number {
            0..=9 => 1,
            10..=99 => 2,
            100..=999 => 3,
            _ => 0,
        }
    }

    fn separator(&self) -> &str {
        match self.number_len() {
            1 => "   | ",
            2 => "  | ",
            3 => " | ",
            4 => "| ",
            _ => {
                info(&format!(
                    "line number {}: unable to include any more line numbers",
                    self.number
                ));
                ""
            }
        }
    }
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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no filename"),
        };

        let ln = match args.next() {
            Some(arg) => {
                if arg == "ln" {
                    true
                } else {
                    info("Invalid argument given for the third parameter");
                    false
                }
            }
            None => false,
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

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
    // we will use `eprintln` for these messages too because we don't want them on the
    // standard output
    // after all, the standard error stream is for diagnostics too!
    eprintln!("\x1b[38;5;11m{}\x1b[0m", string);
}

#[cfg(test)]
mod tests {
    use super::*;

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
