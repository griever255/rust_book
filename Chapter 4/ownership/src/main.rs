fn main() {
    // Variable Scope
    let s = "hello";

    // Listing 4-1: A variable and the scope
    // in which it is valid
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // The String Type
    let s = String::from("hello");
    
    // This string can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    // Listing 4-2: Assigning the integer value of variable
    // x to y
    let x = 5;
    let y = x;
    // Both x and y are on the stack, fixed size in memory
    println!("x = {x}, y = {y}");

    // Strings hold contents: pointer, length, and capacity 
    // ptr: points to the memory on the heap
    // length: how much memory in bytes
    // capacity: total amount of memory in bytes received from allocator
    let s1 = String::from("hello");
    // s2 copies ptr, length, capacity
    // data on heap is unchanged
    let s2 = s1; // this drops s1...double free error
    // since both s1 and s2 would point to same heap memory
    // s1 has moved into s2

    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy (copies heap data)

    println!("s1 = {s1}, s2 = {s2}");

    // Ownership and Functions
    // Listing 4-3: Functions with ownership and scope annotated
    //fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    //} // Here, x goes out of scope, then s. But because s's value was moved, nothing
        // special happens.

    // Listing 4-4: Transferring ownership of return values
    // fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    // } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
         // happens. s1 goes out of scope and is dropped.

    // Listing 4-5: Returning ownership of parameters
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {s2} is {len}");
}

// Listing 4-3 Functions with ownership and scope annotated
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Listing 4-4: Transferring ownership of return values
fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}

// Listing 4-5: Returning ownership of parameters
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}