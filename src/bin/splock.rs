use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct Lock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> Debug for Lock<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }
    fn lock(&self, op: impl FnOnce(&mut T)) {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {}

        op(&mut self.data.borrow_mut());
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let data = Arc::new(Lock::new(0));
    let data1 = data.clone();
    let t1 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        println!("t1");
        data1.lock(|v| *v += 10);
    });
    let data2 = data.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::from_secs(1));
        println!("t2");
        data2.lock(|v| *v *= 10);
    });
    t1.join().unwrap();
    println!("===");
    t2.join().unwrap();
    println!("main data:{:?}", data);
}
