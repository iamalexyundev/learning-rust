fn main() {
    let data = "initial contents";
    //same thing
    let s = String::from(data);
    let s = "initial contents".to_string();

    pushing_strs();
    pushing_char();
    concatenate();
    indexing();
}

fn pushing_strs() {
    //.push_str()
    let mut foo = String::from("foo");
    foo.push_str("bar");
    println!("{foo}");

    //push_str doesnt take ownership
    let mut bar1 = String::from("bar");
    let bar2 = "foo";
    let space = " ".to_string();
    bar1.push_str(&space); // Pushing reference of a String
    bar1.push_str(bar2);
    println!("bar1 is {bar1}");
    println!("bar2 is {bar2}");
}

fn pushing_char() {
    //.push() character
    let mut lo = String::from("lo");
    lo.push('l');
    println!("{lo}");
}

fn concatenate() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let tic = "tic".to_string();
    let tac = "tac".to_string();
    let toe = "toe".to_string();

    let tic_tac_toe_format = format!("{}-{}-{}", tic, tac, toe);
    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    println!("{}", tic_tac_toe);
    println!("{}", tic_tac_toe_format)
}

fn indexing() {
    let hello = "Здравствуйте".to_string();
    //This is not working because some chars can take up more than 1 byte
    // let h = s1[0];

    let s = &hello[0..4];
    println!("{s}");
    for c in s.bytes() {
        println!("{c}")
    }
}
