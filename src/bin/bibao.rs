use std::sync::atomic::{AtomicIsize, Ordering};

fn main() {
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");
    let mut c = || {
        name.push_str(" tyr");
        println!("c:{}", name);
    };
    let mut c1 = move || {
        name1.push_str(" aa");
        println!("c1:{}", name1);
    };
    c();
    c1();
    println!("{name}");
}
