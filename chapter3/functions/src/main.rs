// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }
// fn main() {
//     let my_num = 5;
//     print_labeled_measurement(my_num, 'h');
// }

// fn print_labeled_measurement(value: u32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     let position = (1, 2);
//     another_function(position);
// }
// //Destructuring
// fn another_function((x, y): (i32, i32)) {
//     println!("The value of x and y is: {} and {}", x, y);
// }

// fn five() -> i32 {
//     return 5;
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
