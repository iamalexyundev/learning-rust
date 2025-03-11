use crate::List::{Cons, Nil};
use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}
#[derive(Debug)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!")
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = MyBox::new(42);
    let y = &x;

    println!("{:?}", **y);
    hello("Rust");
    let m = MyBox::new(String::from("Box"));
    hello(&m);
    println!("Initializing CustomSmartPointer!");
    let c = CustomSmartPointer {
        data: String::from("hello"),
    };
}

fn hello(name: &str) {
    println!("hello {name}")
}
