mod numbers;
use numbers::Numbers;
use rand::Rng;
fn main() {
    let vector_length = 100;
    let mut vector = Vec::new();
    for _ in 0..vector_length {
        vector.push(rand::rng().random_range(1..=10));
    }
    let list = Numbers(vector);
    println!("The mean of the list is {}", list.mean());
    println!("The median of the list is {}", list.median());
    println!("The mode of the list is {}", list.mode().unwrap());
    println!("Initial vector is {:?}", &list.0);
}
