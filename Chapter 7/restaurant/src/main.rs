// Listing 7-14: Bringing HashMap into scope in an idiomatic way
use std::collections::HashMap;

use rand::Rng;

// // --snip--
// use std::cmp::Ordering;
// use std::io;
// // --snip--
// Listing 7-18: Specifying a nested path to bring 
// multiple items with the same prefix into scope
use std::{cmp::Ordering, io};

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}