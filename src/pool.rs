use super::HttpPetition;
use super::{debug, error, info};

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread::{self, JoinHandle};

#[allow(dead_code)]
pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    pub sender: Sender<TcpStream>,
    receiver: Arc<Mutex<Receiver<TcpStream>>>,
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> Self {
        let mut threads = Vec::with_capacity(n_threads);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for n in 0..n_threads {
            let receiver_clone = receiver.clone();

            let handler = thread::spawn(move || loop {
                info!("Thread number {} is created", n);
                debug!("Thread number {} awaiting for lock receiver", n);
                let alpha_receiver = receiver_clone.lock().unwrap();
                debug!("Thread number {} listening for connection", n);
                let stream = match alpha_receiver.recv() {
                    Ok(conn) => conn,
                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                };
                drop(alpha_receiver);
                let petition = match HttpPetition::from_conn(stream) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                };
                info!("{:?}", petition);
            });
            threads.push(handler);
        }
        Self {
            threads,
            sender,
            receiver,
        }
    }
}
