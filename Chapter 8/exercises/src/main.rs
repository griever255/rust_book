use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let integers = vec![1, 2, 3, 4, 5, 4, 9, 2, 1, 5, 5, 5];
    let result: Vec<i32> = get_median_and_mode(&integers);
    let median = result.get(0).unwrap();
    let mode = result.get(1).unwrap();
    println!("For the vector {integers:?} \n\
    Median: {median} \n\
    Mode: {mode}");

    let text = "the quick brown fox jumped over the lazy brown dog";
    let pig_text = pig_latin_from_string(&text);
    println!("{text:?} to pig latin is {pig_text:?}");
    employee_interface()
}

fn get_median_and_mode(integers: &Vec<i32>) -> Vec<i32> {
    let mut counts = HashMap::new();
    let mut mode_count = 0;
    let mut mode = 0;
    for i in integers {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
        if *count >= mode_count {
            mode = *i;
            mode_count = *count
        }
    }
    let median = integers[integers.len() / 2];
    vec![median, mode]
}

fn pig_latin_from_string(input: &str) -> String {
    let mut result = "".to_string();
    for word in input.split_whitespace() {
        let first_char = &word[0..1];
        if is_vowel(first_char) {
            result.push_str(&(format!("{word}-hay")));
        } else {
            let remaining_chars = &word[1..];
            result.push_str(&(format!("{remaining_chars}-{first_char}ay")));
        }
        result.push_str(" ");
    }
   result.trim().to_string()
}

fn is_vowel(c: &str) -> bool {
    "AEIOUaeiou".contains(c)
}

fn employee_interface() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!("Enter a command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        while input.trim() == "" {
            if input == "\r\n" || input == "\n" {
                input = String::new();
                print!("Commands are Add, List, or Quit\r\n");
                print!("Enter a command: ");
                io::stdout().flush().unwrap();
            }
            io::stdin().read_line(&mut input).expect("Error reading line");
        }
        let command: String = input
            .trim()
            .parse::<String>()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        match &command[..] {
            "Add" => add(&mut directory, &input.trim()[4..]),
            "List" => list(&mut directory, &input.trim()[5..]),
            "Quit" => break,
            _ => println!("Commands are Add, List, or Quit"),
        }
    }
}

fn add(directory: &mut HashMap<String, Vec<String>>, input: &str) {
    let values = input.trim().split_whitespace();
    if values.clone().count() == 3 {
        if values.clone().nth(1) == Some("to") {
            let key = values.clone().nth(2).unwrap().to_string();
            let person = values.clone().nth(0).unwrap().to_string();
            let employees: &mut Vec<String> = directory.entry(key).or_insert(vec![]);
            if !employees.contains(&person) {
                employees.push(person);
            }
        }
    }
}

fn list(directory: &mut HashMap<String, Vec<String>>, input: &str) {
    let values = input.trim().split_whitespace();
    if values.clone().count() == 1 {
        let key = values.clone().nth(0).unwrap().to_string();
        if directory.contains_key(&key) {
            let value = directory.get(&key).unwrap();
            let mut sorted = value.to_vec();
            sorted.sort();
            println!("The employees in {key} are {sorted:?}");
        } else if key == "all" {
            println!("All people in the company by department are: {directory:?}");
        } else {
            println!("There is no {key} department");
        }
    } else {
        println!("The syntax for List is List $department_name or use keyword all");
    }
}

