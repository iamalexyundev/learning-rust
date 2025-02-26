use std::char;

fn main() {
    let sentence = String::from("hello world apple square"); //->ello-hay orld-way
    let pig_latin_sentence = pig_latin(&sentence);
    println!("{}", pig_latin_sentence)
}

fn pig_latin(sentence: &str) -> String {
    let mut pig_latin_sentence = String::new();
    for word in sentence.split_whitespace() {
        let mut word = word.to_string();
        let first_letter = word.remove(0);
        if is_vovel(first_letter) {
            pig_latin_sentence.push_str(&(first_letter.to_string() + &word + "-hay"));
        } else {
            word.push('-');
            word.push(first_letter);
            pig_latin_sentence.push_str(&(word.to_string() + "ay"));
        }
        pig_latin_sentence.push(' ');
    }
    pig_latin_sentence
}

fn is_vovel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
