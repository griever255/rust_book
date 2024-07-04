use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Listing 16-12: Exploring the API of Mutex<T> in a single-threaded
    // context for simplicity
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    // Listing 16-13: Ten threads each increment a counter guarded by
    // a Mutex<T>
    // Listing 16-15: Using an Arc<T> to wrap the Mutex<T> to be able
    // to share ownership across multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Listing 16-14: Attempting to use Rc<T> to allow multiple
        // threads to own the Mutex<T>
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
