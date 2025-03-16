fn main() {
    type Kilometers = i32;
    let x = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));
}
