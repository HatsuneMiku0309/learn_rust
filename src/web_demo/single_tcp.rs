use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, time::Duration, thread, sync::{mpsc::{self, Receiver}, Arc, Mutex}};

pub fn web_s() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for  stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = rx.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool { workers, sender: tx }
    }
    
    fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

enum Message {
    NewJob(Job),
    Terminate
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn web_m() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for  stream in listener.incoming().take(2) {
    for  stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    let (status_line, html) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK",  " \
        <!DOCTYPE html> \
        <html lang=\"en\"> \
            <head> \
                <meta charset=\"utf-8\"> \
                <title>Hello!</title> \
            </head> \
            <body> \
                <h1>Hello!</h1> \
                <p>Hi from Rust</p> \
            </body> \
        </html> \
        ".trim())
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK",  " \
        <!DOCTYPE html> \
        <html lang=\"en\"> \
            <head> \
                <meta charset=\"utf-8\"> \
                <title>Hello!</title> \
            </head> \
            <body> \
                <h1>Hello!</h1> \
                <p>Hi from Rust</p> \
            </body> \
        </html> \
        ".trim())
    } else {
        ("HTTP/1.1 404 NOT FOUND",  " \
        <!DOCTYPE html> \
        <html lang=\"en\"> \
            <head> \
                <meta charset=\"utf-8\"> \
                <title>Hello!</title> \
            </head> \
            <body> \
                <h1>Oops!</h1> \
                <p>Sorry, I don't know what you're asking for.</p> \
            </body> \
        </html> \
        ".trim())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html.len(),
        html
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}