use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    // Listing 20-1: Listening for incoming streams and printing a
    // message when we receive a stream
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Listing 20-12: Our ideal ThreadPool interface
    let pool = ThreadPool::build(4).unwrap();
    // Listing 20-25: Shut down the server after serving two requests by exiting the loop
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })

        // Listing 20-11: Spawning a new thread for each stream
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
    }
    
    println!("Shutting down.");
}

// Listing 20-2: Reading from the TcpStream and printing the data
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // Listing 20-6: Handling requests to / differently from other
    // requests
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Listing 20-9: Refactoring the if and else blocks to contain only
    // the code that differs between the two cases
    // Listing 20-10: Simulating a slow request by sleeping for 5 seconds
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();

    // if request_line == "GET / HTTP/1.1" {
    //     // Listing 20-3: Writing a tiny successful HTTP response to the
    //     // stream
    //     // Listing 20-5: Sending the contents of hello.html as the body of
    //     // the response
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();

    //     let response = 
    //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     // Listing 20-7: Responding with status code 404 and an error
    //     // page if anything other than / was requested
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
    //     stream.write_all(response.as_bytes()).unwrap();
    // }
}