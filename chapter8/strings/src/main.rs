fn main() {
    let data = "initial contents";
    //same thing
    let s = String::from(data);
    let s = "initial contents".to_string();

    //.push_str()
    let mut foo = String::from("foo");
    foo.push_str("bar");
    println!("{foo}");

    //push_str doesnt take ownership
    let mut bar1 = String::from("bar");
    let bar2 = "foo";
    bar1.push_str(bar2);
    println!("bar2 is {bar2}");

    let mut lo = String::from("lo");
    lo.push('l');
    println!("{lo}")
}
