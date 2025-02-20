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
    println!("{}", value_in_cents(coin));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the max config is {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("the max config is {max}")
    } else {
        println!("NOT MAX")
    }
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
