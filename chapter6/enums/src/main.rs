enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(1, 1, 1, 1);

    let office = IpAddrKind::V6(String::from("1:1:1"));

    let m = Message::Write(String::from("HELLO"));
    m.call();

    let some_number = Some(5);
    let some_str = Some("string slice");

    let absent_number: Option<i32> = None;
    println!("{}", some_number.is_none());
}
