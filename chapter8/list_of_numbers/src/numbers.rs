use std::collections::HashMap;

pub struct Numbers(pub Vec<i32>);

impl Numbers {
    pub fn mean(&self) -> f32 {
        if self.0.is_empty() {
            return 0 as f32;
        }
        self.0.iter().sum::<i32>() as f32 / self.0.len() as f32
    }

    pub fn median(&self) -> f32 {
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
    pub fn mode(&self) -> Option<i32> {
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
