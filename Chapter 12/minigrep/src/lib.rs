// Listing 12-13: Moving Config and run into src/lib.rs
use std::fs;
use std::error::Error;
use std::env;

// Listing 12-6: Refactoring parse_config to return an
// instance of a Config struct
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Listing 12-7: Changing parse_config into Config::new
impl Config {
    // Listing 12-5: Extracting a parse_config function from main
    // Listing 12-9: Returning a Result from Config::build
    // Listing 13-19: Updating the signature of Config::build 
    // to expect an iterator
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        // Listing 12-8: Adding a check for the number of arguments
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // Listing 13-20: Changing the body of Config::build to
        // use iterator methods
        let query = match args.next() { //args[1].clone();
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {  //args[2].clone();
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // Listing 12-23: Checking for any value in an
        // environment variable named IGNORE_CASE
        let ignore_case = match args.next() {
            Some(arg) => match arg.as_str() {
                "true" => true,
                "false" => false,
                "1" => true,
                "0" => false,
                _ => return Err("ignore_case must be true/false"),
                },
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}

// Listing 12-11: Extracting a run function containing
// the rest of the program logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Listing 12-4: Reading the contents of the file
    // specified by the second argument
    // Listing 12-12: Changing the run function to return Result
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");

    // Listing 12-22: Calling either search or search_case_insensitive
    // based on the value in config.ignore_case
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// Listing 12-16: Defining just enough of the search function 
// so our test will compile
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // // Listing 12-17: Iterating through each line in contents
    // for line in contents.lines() {
    //     // Listing 12-18: Adding functionality to see whether
    //     // the line contains the string in query
    //     if line.contains(query) {
    //         // Listing 12-19: Storing the lines that match 
    //         // so we can return them
    //         results.push(line);
    //     }
    // }
    // results

    // Listing 13-20: Changing the body of Config::build to use
    // iterator methods
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Listing 12-21: Defining the search_case_insensitive function to lowercase
    // the query and the line before comparing them
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Listing 12-15: Creating a failing test for the search
    // function we wish we had
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // Listing 12-20: Adding a new failing test for
    // the case-insensitive function we’re about to add
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}