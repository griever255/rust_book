fn main() {
    // Listing 15-6: Using the dereference operator to follow
    // a reference to an i32 value
    let x = 5;
    // Listing 15-7: Using the dereference operator on a Box<i32>
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T>