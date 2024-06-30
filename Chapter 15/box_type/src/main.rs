use crate::List::{Cons, Nil};

// Listing 15-2: The first attempt at defining an enum to 
// represent a cons list data structure of i32 values
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Listing 15-1: Storing an i32 value on the heap using
    // a box
    let b = Box::new(5);
    println!("b = {b}");   

    // Listing 15-3: Using the List enum to store the list
    // 1, 2, 3
    // Listing 15-5: Definition of List that uses Box<T> in
    // order to have a known size
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
