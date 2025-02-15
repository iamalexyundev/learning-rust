use std::io;
fn main() {
    // let input = "42";
    // let guess: u8 = input.parse().expect("Not a number!");
    // println!("The guess is {guess}");
    // let x = 0.1;
    // let y: f32 = 2.3;
    // println!("X is {x}, Y is {y}, {}", x + y)

    // //Numeric operations
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;
    // let remainder = 43 % 5;

    // // The Boolean type
    // let t = true;
    // let f = false;
    // let c = 'z';
    // let heart_eyed_cat = '😻';

    //Compound types: tuples and arrays
    //Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // let my_collection = (1, 'a', "hello world!");
    // println!(
    //     "{} : {} : {} : {:?}",
    //     my_collection.0, my_collection.1, my_collection.2, my_collection
    // );
    // //Array
    // let array = [1, 2, 3, 4, 5];
    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];
    // let arr = [3; 5];
    // let first = array[0];
    // let second = array[1];
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
