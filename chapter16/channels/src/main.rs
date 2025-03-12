use std::{sync::mpsc, thread, time::Duration};
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        println!("val is {val}");
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}")
}
