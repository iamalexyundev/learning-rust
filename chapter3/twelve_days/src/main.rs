fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..days.len() {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[i]
        );
        for gift in (0..=i).rev() {
            println!("{}", gifts[gift]);
        }
        // let mut gift = idx as i32;
        // while gift >= 0 {
        //     println!("{}", gifts[gift as usize]);
        //     gift -= 1;
        // }
        println!("");
    }
}
