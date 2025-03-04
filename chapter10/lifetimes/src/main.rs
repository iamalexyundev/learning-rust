#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("the longest string is {result}");
    }

    let sentence = String::from("This is the first part. This is the second part.");
    let first_part = sentence.split(".").next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt { part: first_part };

    println!("The excerpt is: {:?}", excerpt.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
