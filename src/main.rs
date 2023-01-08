fn pig_latin(word: &str) -> String {
    // this vector is built of type vec<&str> instead of vec<char> intentionally,
    // to make it easier to check if it contains the first word letter or not,
    // as on creating a slice out of the word it will be of type &str
    let vowels = vec!["A", "a", "E", "e", "I", "i", "O", "o", "U", "u", "Y", "y"];
    // converting the &str to String
    // so string methods are accessible on our value
    let mut word = word.to_string();

    // if the word starts with a vowel
    if vowels.contains(&&word[..1]) {
        word.push_str("hay");

    // if the word starts with consonant
    } else {
        word = format!("{}{}ay", &word[1..], &word[..1]);
    };

    return word;
}

fn main() {
    let word = pig_latin("word");

    println!("{word}");
}
