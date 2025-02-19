struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("john doe"),
        email: String::from("johndoe@gmail.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    user1.email = String::from("johndoeburner@gmail.com");
    println!("{}", user1.email)
}
