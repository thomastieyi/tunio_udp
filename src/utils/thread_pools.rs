use std::sync::mpsc::{Sender, self};
use std::sync::{Arc, Mutex};
use std::thread;

// use futures::channel::mpsc::{Sender, self};

pub(crate) struct _ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    sender: Sender<_Message>,
}

type _Job = Box<dyn FnOnce() + Send + 'static>;

enum _Message {
    NewJob(_Job),
    Terminate,
}

impl _ThreadPool {
    /// 创建线程池。
    ///
    /// `size`是线程池中的线程数。
    pub fn _new(size: usize) -> _ThreadPool {
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
                        _Message::NewJob(job) => {
                            println!("Handling job in worker thread");
                            job();
                        }
                        _Message::Terminate => break,
                    }

                    },
                    Err(_e) => {
                        // println!("e");

                      // 处理错误 
                    }
                  }


            }));
        }

        _ThreadPool { threads, sender }
    }

    /// 执行一个任务
    pub fn _execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.send(_Message::NewJob(job)).unwrap();
    }
}