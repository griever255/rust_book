// Listing 16-1: Creating a new thread to print one thing while the
// main thread prints something else
use std::thread;
use std::time::Duration;

fn main() {
    // Listing 16-2: Saving a JoinHandle from thread::spawn to 
    // guarantee the thread is run to completion
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Listing 16-3: Attempting to use a vector created by the main
    // thread in another thread
    let v = vec![1, 2, 3];

    // Listing 16-5: Using the move keyword to force a closure to
    // take ownership of the values it uses
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

}
