use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let min_possible_secret = 1;
    let max_possible_secret = 100;
    let range = min_possible_secret..=max_possible_secret;
    println!("Welcome to the reverse guessing game!");
    loop {
        println!("Please give me a secret number between 1 and 100");
        let mut secret_number = String::new();
        io::stdin()
            .read_line(&mut secret_number)
            .expect("Failed to get the input");
        let secret_number: u32 = match secret_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        if !range.contains(&secret_number) {
            println!("The number is out of range");
            continue;
        }
        let mut guess = 50;
        let mut last_guess_change_amount = 0;
        loop {
            println!("Computer guess is {guess}");
            match guess.cmp(&secret_number) {
                Ordering::Equal => {
                    println!("I won!");
                    break;
                }
                Ordering::Less => {
                    println!("Too small");
                    if last_guess_change_amount == 0 {
                        last_guess_change_amount = max_possible_secret - guess;
                    } else {
                        last_guess_change_amount = last_guess_change_amount / 2;
                    }
                    guess = guess + last_guess_change_amount;
                }
                Ordering::Greater => {
                    println!("Too big");
                    if last_guess_change_amount == 0 {
                        last_guess_change_amount = guess / 2;
                    } else {
                        last_guess_change_amount = last_guess_change_amount / 2;
                    }
                    guess = guess - last_guess_change_amount;
                }
            }
        }
        println!("Secret nubmer {secret_number}");
        break;
    }
}
