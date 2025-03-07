// use std::{sync::{mpsc, Arc, Mutex}, thread};

use surrealdb::{engine::remote::ws::Client, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Error;
pub struct DataBase{
    _db: Surreal<Client>,
}

impl DataBase {
    pub async fn sign_in(username: &str, password: &str) -> Result<DataBase, Error>{
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        db.signin(Root {
            username,
            password
        }).await?;

        db.use_ns("test").use_db("test").await?;
        Ok(DataBase { _db: db })
    }
}

// pub struct ThreadPool{
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     fn new(id:usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let message = receiver.lock().unwrap().recv().unwrap();

//             println!("Working on job {}.", id);

//             match message {
//                 Message::NewJob(job) => {
//                     job();
//                 }
//                 Message::Terminate => {
//                     println!("Worker {} is terminating work.", id);
//                     break;
//                 }
//             }
//         });

//         Worker{ id, thread: Some(thread) }
//     }
// }

// type Job = Box<dyn FnOnce() + Send + 'static>;

// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// impl ThreadPool {
//     pub fn new(size:usize) -> ThreadPool{
//         assert!(size > 0);

//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));
//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         ThreadPool { workers, sender }
//     }

//     pub async fn execute<F>(&self, f:F)
//     where
//         F: FnOnce() + Send + 'static
//         {
//             let job = Box::new(f);
//             self.sender.send(Message::NewJob(job)).unwrap();
//         }
// }

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         for worker in &mut self.workers {
//             println!("Shutting down worker {}", worker.id);

//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }