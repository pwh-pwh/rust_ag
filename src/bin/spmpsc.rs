use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::atomic::AtomicIsize;
use std::sync::{Arc, Condvar, Mutex};

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicIsize,
    receivers: AtomicIsize,
}

struct Sender<T> {
    shared: Arc<Shared<T>>,
}

struct Received<T> {
    sender: Arc<Shared<T>>,
}

fn main() {}
