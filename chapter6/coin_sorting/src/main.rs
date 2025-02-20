#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    California,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::California);
    println!("{}", value_in_cents(coin))
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Your quarter is from {:?}", state);
            25
        }
    }
}
