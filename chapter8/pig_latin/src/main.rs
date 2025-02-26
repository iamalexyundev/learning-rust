fn main() {
    let sentence = String::from("hello world apple square"); //->ello-hay orld-way
    let pig_latin_sentence = pig_latin(&sentence);
    println!("{}", pig_latin_sentence)
}

fn pig_latin(sentence: &str) -> String {
    let mut pig_latin_sentence = String::new();
    for word in sentence.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        let pig_latin_word = match chars[0] {
            'a' | 'e' | 'i' | 'o' | 'u' => word.to_string() + "-hay",
            _ => {
                let first_letter = chars.remove(0);
                let mut word: String = chars.iter().collect();
                word += "-";
                word.push(first_letter);
                word + "ay"
            }
        };
        pig_latin_sentence.push_str(&pig_latin_word);
        pig_latin_sentence.push(' ');
    }
    pig_latin_sentence
}
