

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new())); // 创建 Mutex 和 CondVar 对象

    let pair_clone = pair.clone(); // 克隆 Arc 对象以便在新线程中使用
    thread::spawn(move || {
        let (lock, cvar) = &*pair_clone; // 获取 Mutex 和 CondVar 对象的引用
        let mut done = lock.lock().unwrap();
        eprintln!("zi lock");
        *done = true;
        // sleep(Duration::from_secs(1));
        cvar.notify_one(); // 唤醒等待的线程
        sleep(Duration::from_secs(1));
        // drop(done);
        loop {

        }
    });

    let (lock, cvar) = &*pair; // 获取 Mutex 和 CondVar 对象的引用
    // sleep(Duration::from_secs(1));
    let mut done = lock.lock().unwrap();
    eprintln!("done");
    while !*done {
        eprintln!("1");
        done = cvar.wait(done).unwrap(); // 阻塞当前线程，并等待唤醒信号
        eprintln!("2");
    }
    eprintln!("bb")
}
