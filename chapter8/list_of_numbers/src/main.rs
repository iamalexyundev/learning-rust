fn main() {
    //Create a list of numbers
    //TODO: Generate list of random number unsorted
    let list = vec![2, 2, 3, 4, 5];
    println!("The mean of the list {:?} is {}", &list, get_mean(&list))
}

fn get_mean(list: &Vec<i32>) -> f32 {
    let mut total: f32 = 0.0;
    for num in list {
        total += *num as f32
    }
    total / list.len() as f32
}
