use std::{sync::{mpsc, Arc, Mutex}, fmt, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

// Listing 20-15: Modifying ThreadPool to hold Worker
// instances instead of holding threads directly
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Listing 20-17: Passing the receiver to the workers
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        // Listing 20-20: Receiving and executing the jobs in the worker’s thread
        let thread = thread::spawn( move || loop { 
            // Listing 20-24: Explicitly break out of the loop when recv returns an error
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job(); 
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    fn new(size: usize) -> ThreadPool {
        // Listing 20-13: Implementing ThreadPool::new to panic if size is zero
        // assert!(size > 0);
        
        // Listing 20-16: Modifying ThreadPool to store the sender of a channel
        // that transmits Job instances
        let (sender, receiver) = mpsc::channel();
        
        // Listing 20-18: Sharing the receiver among the workers using
        // Arc and Mutex
        let receiver = Arc::new(Mutex::new(receiver));

        // Listing 20-14: Creating a vector for ThreadPool to hold the
        // threads
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => return Err(PoolCreationError {size,}),
            _ => return Ok(ThreadPool::new(size)),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Listing 20-19: Creating a Job type alias for a Box that
        // holds each closure and then sending the job down the channel
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Listing 20-22: Joining each thread when the thread pool goes out of
// scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Listing 20-23: Explicitly drop sender before joining the worker threads
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct PoolCreationError {
    size: usize,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't create a ThreadPool of size {}", self.size)
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't create a ThreadPool of size {}", self.size)
    }
}