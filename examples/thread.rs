use std::sync::mpsc::{Sender, self};
use std::sync::{Arc, Mutex};
use std::thread;

// use futures::channel::mpsc::{Sender, self};

struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /**
     * 创建线程池, size是线程池中的线程数, 返回线程池, 用于执行任务, 任务是一个闭包, 闭包中的内容是要执行的任务
     * 
     */
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
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Handling job in worker thread");
                        job();
                    }
                    Message::Terminate => break,
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

fn main() {
    let pool = ThreadPool::new(4);
    
    for i in 1..10 {
        let closure = move || {
            println!("hello from job {}", i);
        };

        pool.execute(closure);
    }

    drop(pool); // 等待所有任务完成
}