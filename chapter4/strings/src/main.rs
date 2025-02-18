// fn main() {
//     // Growing string
//     let mut s = String::from("hello");
//     s.push_str(", world!");
//     println!("{}", s);

//     // bind the value 5 to x; then make a copy of the value in x and bind it to y
//     let mut x = 6;
//     let y = x;
//     x = x + 4;
//     //if i modify x, y is still 6
//     println!("{} {}", x, y);

//     //Example with a string
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s2}, world!");
// }
//Taking ownership
// fn main() {
//     let s = String::from("string");

//     takes_ownership(s);
//     //not allowed to use s anymore, because value is dropped
//     let x = 5;
//     makes_copy(x);
//     //allowed to use x, because value is copied to fn, because i32 implements Copy trait
// }
// fn takes_ownership(some_string: String) {
//     println!("{some_string}")
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}")
// }
//Giving ownership
// fn main() {
//     let s1 = gives_ownership();
//     let s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2);
// }
// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
