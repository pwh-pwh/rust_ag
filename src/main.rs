mod solution;

use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;

fn main() {
    let mut result = File::create("aa.txt").unwrap();
    let w: &mut dyn Write = &mut result;
    w.write_all(b"bb").unwrap();
    let r = Arc::new(1);
    let b = r.clone();
    thread::spawn(move || {
        println!("{:?}", b);
    });
    struct A {
        name: String,
    }
    let a = A {
        name: Default::default(),
    };
    let a: i32 = At::f();
    println!("{}", a);
    let s = "a".to_string();
    let ss = &s;
    let s1 = ss.deref();
    let st = MyString { inner: 1 };
    let st1: i32 = *st;
}
impl At for i32 {
    fn f() -> Self {
        20
    }
}
struct MyString {
    inner: i32,
}

impl Deref for MyString {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &2
    }
}
trait At {
    fn f() -> Self;
}
