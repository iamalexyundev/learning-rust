use rand::Rng;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess}, the secret number: {secret_number}");
}
