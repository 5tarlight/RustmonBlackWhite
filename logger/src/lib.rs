use std::{
    io::{self, Write},
    sync::mpsc,
    thread::{self, JoinHandle},
};

pub struct Logger {
    pub sender: mpsc::Sender<String>,
    pub receiver: mpsc::Receiver<String>,
}

impl Logger {
    pub fn new() -> (mpsc::Sender<String>, JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel::<String>();

        let handle = thread::spawn(move || loop {
            for msg in receiver.try_iter() {
                println!("{}", msg);
                io::stdout().flush().unwrap();
            }
        });

        (sender, handle)
    }
}
