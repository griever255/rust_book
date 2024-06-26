use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

// Listing 9-12: Changing main to return Result<(), E> 
// allows the use of the ? operator on Result values
fn main() -> Result<(), Box<dyn Error>> {
    // Listing 9-3: Opening a file
    let greeting_file_result = File::open("hello.text");

    // Listing 9-4: Using a match expression to handle 
    // the Result variants that might be returned
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Listing 9-5: Handling different kinds of errors 
        // in different ways
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            }
            other_error => {
                panic!("Problem opening the file: {error:?}");
            }
        }
    };

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    Ok(())
}


fn read_username_from_file() -> Result<String, io::Error> {
    // Listing 9-6: A function that returns errors to the calling
    // code using match
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Listing 9-7: A function that returns errors to the 
    // calling code using the ? operator
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    //Listing 9-8: Chaining method calls after the ? operator
    // let mut username = String::new();

    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Ok(username)

    // Listing 9-9: Using fs::read_to_string instead of opening and then reading the file
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}