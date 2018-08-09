use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = search(&config.query, &contents, config.case_sensitive);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if case_sensitive {
                line.contains(query)
            } else {
                line.to_lowercase().contains(&query.to_lowercase())
            }
        })
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        match args {
            [_, query, filename] => Ok(Config {
                query: query.clone(),
                filename: filename.clone(),
                case_sensitive,
            }),

            [_, flag, query, filename] if flag == "-i" => Ok(Config {
                query: query.clone(),
                filename: filename.clone(),
                case_sensitive: false,
            }),

            _ => Err("invalid amount of arguments or wrong argument order"),
        }
    }

    // Iterator based solution from functional programming chapter in the book
    // TODO: expand this solution to support flags (at any arguments list position)
    pub fn new_iter(mut args: std::env::Args) -> Result<Config, &'static str> {
        // first argument is always name of the program, ignore it
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
