use std::collections::HashMap;
fn main() {
    create_hash_map_and_get_item();
    zip_hash_map();
    overwriting_value();
    updating();
}

fn create_hash_map_and_get_item() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    println!("{:?}", scores);

    let team = "Blue".to_string();
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{:?}", score);
    println!("--------");
    for (key, value) in scores {
        println!("{key}: {value}")
    }
    println!("--------");
}

fn zip_hash_map() {
    let teams = vec!["blue".to_string(), "yellow".to_string()];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores).collect();
    println!("{:?}", scores)
}

fn overwriting_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

fn updating() {
    let text = "hello world wonderful world".to_string();

    let mut map = HashMap::new();
    for word in text.split(" ") {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}")
}
