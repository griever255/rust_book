// Listing 7-21: Declaring the front_of_house module whose body 
// will be in src/front_of_house.rs
mod front_of_house; 

// Listing 7-8: Calling a function using a relative path starting 
// with super
fn deliver_order() {}

mod back_of_house {
    // Listing 7-8: Calling a function using a relative path 
    // starting with super
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // Listing 7-9: A struct with some public fields 
    // and some private fields
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Listing 7-10: Designating an enum as public makes all 
    // its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Listing 7-17: Making a name available for any code to use 
// from a new scope with pub use
// Listing 7-11: Bringing a module into scope with use
pub use crate::front_of_house::hosting;

// Listing 7-13: Bringing the add_to_waitlist function into 
// scope with use, which is unidiomatic
use crate::front_of_house::hosting::add_to_waitlist;


// Listing 7-3: Calling the add_to_waitlist function using 
// absolute and relative paths
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    //Listing 7-9: A struct with some public fields and some 
    // private fields
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Listing 7-10: Designating an enum as public makes all its 
    // variants public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    add_to_waitlist();
}

// Listing 7-15: Bringing two types with the same name into 
// the same scope requires using their parent modules.
use std::fmt;
use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Listing 7-16: Renaming a type when it’s brought into scope 
// with the as keyword
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// Listing 7-19: Two use statements where one is a subpath of the other
// use std::io;
// use std::io::Write;

// Listing 7-20: Combining the paths in Listing 
// 7-19 into one use statement
// use std::io::{self, Write};

use std::collections::*;