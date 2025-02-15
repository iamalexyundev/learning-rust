// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }
fn main() {
    let my_num = 5;
    print_labeled_measurement(my_num, 'h');
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
