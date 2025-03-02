use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    let result = read_username_from_file("hello.txt");
    println!("{:?}", result)
}
fn read_username_from_file(file_name: &str) -> Result<String, Error> {
    // let username_file_result = File::open(file_name);
    // //it needs to be mutable to keep track of internal cursor
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(_) => Err(Error::new(ErrorKind::Other, "oh no!")),
    // }

    //does almost the same thing but more shorter
    // let mut username_file = File::open(file_name)?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    //even shorter
    // let mut username = String::new();
    // File::open(file_name)?.read_to_string(&mut username)?;
    // Ok(username)

    //even shorter
    fs::read_to_string(file_name)
}
