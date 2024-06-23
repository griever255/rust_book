// Listing 10-24: A struct that holds a reference,
// requiring a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
// Listing 10-17: Annotations of the lifetimes 
// of r and x, named 'a and 'b, respectively
//     let r;          // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;   // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }                         // ---------+

// Listing 10-18: A valid reference because 
// the data has a longer lifetime than the reference
    let x = 5;         // ----------+-- 'b
                            //           |
    let r = &x;       // --+-- 'a  |
                            //   |       |
    println!("r: {r}");     //   |       |

    // Listing 10-19: A main function that calls the longest
    // function to find the longer of two string slices
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(string1.as_str(), string2, "HELLO");
    println!("The longest string is {result}");
    //&i32        // a reference
    //&'a i32     // a reference with an explicit lifetime
    //&'a mut i32 // a mutable reference with an explicit lifetime

    // Listing 10-22: Using the longest function with references to
    // String values that have different concrete lifetimes
    let string1 = String::from("Long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "HELLO");
        println!("The longest string is {result}");
    }

    // Listing 10-24: A struct that holds a reference,
    // requiring a lifetime annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}                           // ----------+

// Listing 10-20: An implementation of the longest function
//  that returns the longer of two string slices but does
// not yet compile
// Listing 10-21: The longest function definition specifying
// that all the references in the signature must have the same
// lifetime 'a
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}