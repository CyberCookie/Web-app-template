use std::thread;
use std::sync::{ mpsc , Arc, Mutex };


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    id: usize
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(
            thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        job();
                    },
                    Message::Terminate => {
                        break;
                    }
                }
            })
        );
        return Worker { thread, id }
    }
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(pool_size: usize) -> ThreadPool {
        assert!(pool_size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(pool_size);

        for worker_id in 0..pool_size {
            workers.push(
                Worker::new(worker_id, Arc::clone(&receiver))
            );
        }

        return ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}