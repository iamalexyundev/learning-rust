fn main() {
    let mut s = String::from("hello world");
    let s1 = &mut s;
    mutate_string(s1);
    println!("{}", s1,);
    let s2 = &mut s;
    mutate_string(s2);
    println!("{}", s2);
}
fn mutate_string(to_mutate: &mut String) {
    to_mutate.push_str(" mutant");
}
