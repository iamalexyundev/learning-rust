use std::collections::HashMap;

fn main() {
    //Create a list of numbers
    //TODO: Generate list of random number unsorted
    let list = vec![1, 2, 3, 4, 5];
    println!("The mean of the list is {}", get_mean(&list));
    println!("The median of the list is {}", get_median(&list[..])); //&list is pretty much the same
    println!("The mode of the list is {}", get_mode(&list));
    println!("Initial vector is {:?}", &list);
}

fn get_mean(list: &[i32]) -> f32 {
    if list.is_empty() {
        return 0.0;
    }
    let mut total: f32 = 0.0;
    for num in list {
        total += *num as f32
    }
    total / list.len() as f32
}

//&[i32] because Rust converts it into slice automatically
//it is better because this function can also work with slices of type i32
//arrays etc. it makes it more flexible by not accepting only Vec<i32>
fn get_median(list: &[i32]) -> f32 {
    let mut sorted = list.to_vec();
    let length = sorted.len();
    let is_even = length % 2 == 0;
    sorted.sort_unstable();
    let index = length / 2;
    if is_even {
        get_mean(&sorted[index - 1..=index])
    } else {
        sorted[index] as f32
    }
}

fn get_mode(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    let mut frequency_map = HashMap::new();
    let mut most_frequent = (0, 0);
    for number in list {
        let count = frequency_map.entry(*number).or_insert(0);
        *count += 1;
        if *count > most_frequent.1 {
            most_frequent = (*number, *count)
        }
    }
    most_frequent.0
}
