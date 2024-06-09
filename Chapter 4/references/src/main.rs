fn main() {
    // Reference and Borrowing
    let s1 = String::from("hello");
    // refer to s1 without taking ownership
    // &s1 is a pointer to the data in s1
    let len = calculate_length(&s1); 
    println!("The length of {s1} is {len}");

    // Mutable References
    let mut s = String::from("hello");
    // this function will mutate the value is borrows
    // RULE: There can only be one mutable reference to a value
    change(&mut s); 

    // Multiple mutable references are allowed in new scopes
    // just not simultaneous ones
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } //r1 is dropped
    let r2 = &mut s;

    // Also cannot have a mutable reference when there are immutable refernces
    // For example
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem immutable
    // let r2 = &s; // no problem immutable
    // let r3 = &mut s; // BIG PROBLEM because there's immutable references

    // println!("{}, {}, and {}", r1, r2, r3);

    // This works because of scopes. The immutable references
    // are no longer used after the println!
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    // Dangling References
    // This would return a pointer to something that was dropped
    // let reference_to_nothing = dangle(); 

    // This returns the string directly
    let reference_to_something = no_dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped

// this function will mutate the value is borrows
fn change(some_string: &mut String){
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}