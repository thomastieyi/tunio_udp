use std::sync::mpsc::{Sender, self};
use std::sync::{Arc, Mutex};
use std::thread;

// use futures::channel::mpsc::{Sender, self};

pub(crate) struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// `size`是线程池中的线程数。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 创建线程
            let receiver = Arc::clone(&receiver);
            threads.push(thread::spawn(move || loop {
                // let message = receiver.lock().unwrap().recv().unwrap();
                match receiver.lock().unwrap().recv() {
                    Ok(msg) => {
                      // ...
                      match msg {
                        Message::NewJob(job) => {
                            println!("Handling job in worker thread");
                            job();
                        }
                        Message::Terminate => break,
                    }

                    },
                    Err(_e) => {
                        // println!("e");

                      // 处理错误 
                    }
                  }


            }));
        }

        ThreadPool { threads, sender }
    }

    /// 执行一个任务
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}