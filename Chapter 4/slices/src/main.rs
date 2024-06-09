fn main() {
    // The Slice Type
    // Listing 4-8: Sotring the result from calling the first_word
    // function and then changing the String contents
    let mut s = String::from("hello world");
    let word = first_word(&s); // world will get the value 5

    // this won't work because clear() needs a &mut of s
    // while the print statement's {word} is an immutable borrow
    // can't have an immutable and mutable reference in same scope
    // s.clear(); // this empties the String, making it equal to ""

    println!("the first word is: {word}");

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is no totally invalid!

    // Basically the index in word gets out of sync with the data in s

    // String Slices
    let s = String::from("hello world");
    // rather than reference the whole s, reference a portion
    let hello = &s[0..5];
    let world = &s[6..11];

    // these are equal
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // these are equal
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // these are equal
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    // String Literals as Slices
    // the type of s is &str (string slice)
    // immutable reference to a specific point of the binary
    let s = "Hello, world!";

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, 
    // whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, 
    // which are equivalent to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, 
    // whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    // this slice has type &[i32]
    // slices store a reference to the first element and a length
    // they point to a portion of the data
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);

}

// write a function that takes a string of words 
// separated by spaces and returns the first word 
// it finds in that string. If the function doesn’t 
// find a space in the string, the whole string must 
// be one word, so the entire string should be returned.

// this won't work because the referenced String could change
// fn first_word(s: &String) -> usize { // return index to end of word
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// this would work and introduced the string slice type
// fn first_word(s: &String) -> &str { //&str signifies string slice
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// Listing 4-9 Improving the first_word function by using a
// string slice for the type of the s parameter
// Flexibility allows for deref coercions 
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}