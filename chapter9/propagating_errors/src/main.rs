use std::fs::File;
use std::io::{Error, Read};

fn main() {
    let result = read_username_from_file("hello.txt");
    println!("{:?}", result)
}
fn read_username_from_file(file_name: &str) -> Result<String, Error> {
    let username_file_result = File::open(file_name);
    //it needs to be mutable to keep track of internal cursor
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
