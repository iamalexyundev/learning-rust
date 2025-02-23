fn main() {
    let mut empty_vec: Vec<i32> = Vec::new();
    let vec_with_initial_values = vec![1, 2, 3];
    empty_vec.push(1);
    empty_vec.push(2);
    empty_vec.push(3);
    empty_vec.push(4);

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}")
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{row:?}");

    let my_string = String::from("hello world");
    let another_string;
    {
        let mut my_strings = vec![];
        my_strings.push(&my_string);
        another_string = my_strings[0];
    }
}
