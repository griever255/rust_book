use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Listing 16-6: Creating a channel and assigning the two halves
    // to tx and rx
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    // Listing 16-7: Moving tx to a spawned thread and sending “hi”
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();

        // Listing 16-10: Sending multiple messages and pausing
        // between each
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Listing 16-11: Sending multiple messages from multiple producers
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // Listing 16-10: Sending multiple messages and pausing
        // between each
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Listing 16-8: Receiving the value “hi” in the main thread
    // and printing it
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    
    // Listing 16-10: Sending multiple messages and pausing
    // between each
    for received in rx {
        println!("Got: {received}");
    }
}
