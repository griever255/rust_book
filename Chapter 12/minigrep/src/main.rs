use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Listing 12-1: Collecting the command line 
    // arguments into a vector and printing them
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // Listing 12-2: Creating variables to hold
    // the query argument and file path argument
    // Listing 12-10: Exiting with an error code if
    // building a Config fails
    // Listing 12-24: Writing error messages to standard error
    // instead of standard output using eprintln!
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // Listing 12-14: Using the minigrep library
    // crate in src/main.rs
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
