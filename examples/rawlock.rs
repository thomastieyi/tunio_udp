use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
  let data = Arc::new(RwLock::new(0));

  let mut handles = Vec::new();

  for _ in 0..10 {
    let data = data.clone();
    handles.push(thread::spawn(move || {
      // 获取读锁
      let val = data.read().unwrap();
      println!("Read: {}", val);
      drop(val);

      // 获取写锁  
      let mut data = data.write().unwrap();
      *data += 1;
    }));
  }

  for handle in handles {
    handle.join().unwrap();
  }
}