// use std::{sync::{mpsc, Arc, Mutex}, thread};

use serde::Serialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Error;
pub struct DataBase{
    db: Surreal<Client>,
    _signed_in: bool,
}

impl DataBase {
    pub async fn sign_in(username: &str, password: &str) -> Result<DataBase, Error>{
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        db.signin(Root {
            username,
            password
        }).await?;

        db.use_ns("test").use_db("test").await?;
        Ok(DataBase { db, _signed_in: true })
    }

    pub async fn create_application(&self, application:Application) -> Result<Option<Application>, Error>{
        let created: Option<Application> = self.db.create(("application", application.application_id.clone())).content(application).await?;
        Ok(created)
    }

    pub async fn get_all_applications(&self) -> Result<Applications, Error>{
        let applications:Vec<Application> = self.db.select("application").await?;
        Ok(Applications { applications })
    }

    pub async fn update_application(&self, application:Application, old_id: String) -> Result<Option<Application>, Error>{
        let updated:Option<Application> = self.db.update(("application", old_id)).content(application).await?;
        Ok(updated)
    }

    pub async fn delete_application(&self, application:Application) -> Result<Option<Application>, Error>{
        let delete = self.db.delete(("application", application.application_id)).await?;
        Ok(delete)
    }
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub enum Status {
    InProgress,
    Applied,
    Rejected,
    Interviewing
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct Application {
    application_id: String,
    company: String,
    status: Status,
    job_title: String,
    location: String,
    link: String,
    application_date: String,
    tasks: Vec<String>,
}

impl Application {
    pub fn new(application_id:String, company: String, status:Status, job_title:String, location:String, link:String, application_date: String, tasks:Vec<String>) -> Application{
        Application { application_id, company, status, job_title, location, link, application_date, tasks}
    }
}

#[derive(Debug, Serialize, serde::Deserialize)]
pub struct Applications {
    applications:Vec<Application>
}

impl Applications {
    pub fn new() -> Applications {
        let applications:Vec<Application> = Vec::new();
        Applications{ applications }
    }

    pub fn add(&mut self, application: Application) {
        self.applications.push(application);
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