use std::collections::HashMap;

#[derive(Debug)]
struct Numbers(Vec<i32>);

impl Numbers {
    fn mean(&self) -> f32 {
        println!("{:?}", self);
        if self.0.is_empty() {
            return 0 as f32;
        }
        self.0.iter().sum::<i32>() as f32 / self.0.len() as f32
    }

    fn median(&self) -> f32 {
        if self.0.is_empty() {
            return 0 as f32;
        }
        let mut sorted = self.0.to_vec();
        sorted.sort_unstable();
        let len = sorted.len();
        let is_even = len % 2 == 0;
        let mid_idx = len / 2;
        if is_even {
            Numbers(sorted[mid_idx - 1..=mid_idx].to_vec()).mean()
        } else {
            sorted[mid_idx] as f32
        }
    }

    fn mode(&self) -> i32 {
        if self.0.is_empty() {
            return 0;
        }
        let mut frequency_map = HashMap::new();
        let mut most_frequent = (0, 0);
        for number in &self.0 {
            let count = frequency_map.entry(number).or_insert(0);
            *count += 1;
            if *count > most_frequent.1 {
                most_frequent = (*number, *count)
            }
        }
        most_frequent.0
    }
}
fn main() {
    //Create a list of numbers
    //TODO: Generate list of random number unsorted
    let list = Numbers(vec![5, 4, 3, 2, 1]);
    println!("The mean of the list is {}", list.mean());
    println!("The median of the list is {}", list.median());
    println!("The mode of the list is {}", list.mode());
    println!("Initial vector is {:?}", &list);
}
