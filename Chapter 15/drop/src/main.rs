//Listing 15-14: A CustomSmartPointer struct that 
//implements the Drop trait where we would put our cleanup code
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!",
            self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPonters created.");

    // Listing 15-15: Attempting to call the drop method from the
    // Drop trait manually to clean up early
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // Listing 15-16: Calling std::mem::drop to explicitly
    // drop a value before it goes out of scope
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
