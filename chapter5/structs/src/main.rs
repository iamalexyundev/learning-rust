//Regular struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//Tuple struct
struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

#[derive(Debug)]
struct Counts(u32, u32, u32);
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("john doe"),
        email: String::from("johndoe@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("alice@gmail.com"),
        ..user1
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let mut random_numbers = Counts(12, 232, 323);
    println!("{:?}", random_numbers);
    mutate_counts(&mut random_numbers);
    println!("{:?}", random_numbers);
}

fn mutate_counts(count: &mut Counts) {
    count.0 = 1;
    count.1 = 1;
    count.2 = 1;
}
