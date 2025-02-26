use std::collections::HashMap;

#[derive(Debug)]
struct Numbers(Vec<i32>);

impl Numbers {
    fn mean(&self) -> f32 {
        if self.0.is_empty() {
            return 0 as f32;
        }
        self.0.iter().sum::<i32>() as f32 / self.0.len() as f32
    }

    fn median(&self) -> f32 {
        if self.0.is_empty() {
            return 0 as f32;
        }
        let mut sorted = self.0.clone();
        sorted.sort_unstable();

        let len = sorted.len();
        let mid_idx = len / 2;

        match sorted.len() % 2 {
            0 => (sorted[mid_idx - 1] + sorted[mid_idx]) as f32 / 2.0,
            _ => sorted[mid_idx] as f32,
        }
    }

    fn mode(&self) -> Option<i32> {
        if self.0.is_empty() {
            return None;
        }
        let mut frequency_map = HashMap::new();

        self.0.iter().for_each(|&num| {
            *frequency_map.entry(num).or_insert(0) += 1;
        });

        frequency_map
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(num, _)| num)
    }
}
fn main() {
    //Create a list of numbers
    //TODO: Generate list of random number unsorted
    let list = Numbers(vec![5, 4, 3, 2, 1, 5, 5, 5]);
    println!("The mean of the list is {}", list.mean());
    println!("The median of the list is {}", list.median());
    println!("The mode of the list is {}", list.mode().unwrap());
    println!("Initial vector is {:?}", &list);
}
