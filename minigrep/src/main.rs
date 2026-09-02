use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_flag = args.iter().any(|arg| arg == "--ignore-case");
        let sensitive_flag = args.iter().any(|arg| arg == "--case-sensitive");

        if ignore_flag && sensitive_flag {
            return Err("cannot use --ignore-case and --case-sensitive together");
        }

        let ignore_case = if ignore_flag {
            true
        } else if sensitive_flag {
            false
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // this handles the error when > symbol is wrongly print the error message to a file instead of console 
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_flags() {
        let args = vec!["--ignore-case".to_string(), "--case-sensitive".to_string()];

        let result = Config::build(&args);

        assert!(result.is_err());
    }

    #[test]
    fn flag_overrides() {
        unsafe {
            env::set_var("IGNORE_CASE", "1");
        }

        let args = vec![
            "minigrep".to_string(),
            "duct".to_string(),
            "poem.txt".to_string(),
            "--case-sensitive".to_string(),
        ];

        let result = Config::build(&args).unwrap();

        assert_eq!(result.ignore_case, false);

        unsafe {
            env::remove_var("IGNORE_CASE");
        }
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
