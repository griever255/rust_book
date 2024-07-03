fn main() {
    // Listing 15-6: Using the dereference operator to follow
    // a reference to an i32 value
    let x = 5;
    // Listing 15-7: Using the dereference operator on a Box<i32>
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Listing 15-9: Attempting to use MyBox<T> in the same
    // way we used references and Box<T>
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Listing 15-12: Calling hello with a reference to a
    // MyBox<String> value, which works because of deref
    // coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Listing 15-13: The code we would have to write if
    // Rust didn’t have deref coercion
    hello(&(*m)[..]);
}

use std::ops::Deref;
// Listing 15-8: Defining a MyBox<T> type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Listing 15-10: Implementing Deref on MyBox<T>
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Listing 15-11: A hello function that has the parameter name
// of type &str
fn hello(name: &str) {
    println!("Hello, {name}!");
}
