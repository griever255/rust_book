use std::collections::HashMap;

fn main() {
    // Listing 8-20: Creating a new hash map and inserting 
    // some keys and values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Listing 8-21: Accessing the score for the Blue team 
    // stored in the hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Listing 8-22: Showing that keys and values are owned 
    // by the hash map once they’re inserted
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, 
    //try using them and see what compiler error you get!

    // Overwriting a value
    // Listing 8-23: Replacing a value stored with a particular 
    // key
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); 
    println!("{scores:?}");

    // Adding a key and value only if a key isn't prsent
    // Listing 8-24: Using the entry method to only insert 
    // if the key does not already have a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // Updating a value based on an old value
    // Listing 8-25: Counting occurrences of words using a 
    // hash map that stores words and counts
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

}  
