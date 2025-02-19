fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); //this is immutable
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    //why reference to item?
    //kinda got it...
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// fn main() {
//     let s = String::from("hello world");
//     //can do both [..5] or [0..5];
//     let hello = &s[0..5];
//     let world = &s[6..11];
// }
