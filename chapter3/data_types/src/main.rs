fn main() {
    let input = "42";
    let guess: u8 = input.parse().expect("Not a number!");
    println!("The guess is {guess}");
    let x = 0.1;
    let y: f32 = 2.3;
    println!("X is {x}, Y is {y}, {}", x + y)
}
