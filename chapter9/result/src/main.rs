use std::{
    fs::File,
    io::{ErrorKind, Read},
};
fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error {error:?}"),
            },
            other_error => {
                panic!("Problem oppening file: {other_error:?}")
            }
        },
    };
    println!("{:?}", greeting_file);

    let panic_on_openning =
        File::open("panic.txt").expect("panic.txt should be included in this project");
}
