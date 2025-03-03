pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("Largest from extraxted {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("Largest from extraxted {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result_i32 = largest(&number_list);
    let result_char = largest(&char_list);
    println!("Generic i32: {} char: {}", result_i32, result_char);
    println!("----------------------");
}
fn largest_i32(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number
        }
    }
    largest
}

fn largest_char(list: &Vec<char>) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}
