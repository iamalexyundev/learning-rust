use std::collections::HashMap;
fn main() {
    let mut departments = HashMap::new();
    departments.insert("Engineering".to_string(), "Alex".to_string());
    println!("{:?}", departments)
}
