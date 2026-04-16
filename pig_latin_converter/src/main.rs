// Convert strings to Pig Latin.
// The first consonant of each word is moved to the end of the word and ay is added (first becomes irst-fay)
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay)

use std::io;

fn main() {
    println!("Tell me the word you want to convert to Pig Latin! Or enter x to exit!");
    let mut input_word = String::new();
    loop {
        input_word.clear();

        match io::stdin().read_line(&mut input_word) {
            Ok(_) => input_word.truncate(input_word.trim_end().len()),
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        }

        if input_word == "x" {
            break;
        }

        println!("You wrote {}", input_word);
        println!("{}", pig_latin_converter(&input_word));
    }
    println!("See you later!");
}

fn pig_latin_converter(input_word: &str) -> String {
    let vowels = vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];

    if input_word.is_empty() {
        println!("Please enter a valid word!");
        return String::new();
    }

    if vowels.iter().any(|&v| input_word.starts_with(v)) {
        // This is a word that starts with a vowel
        format!("The Pig Latin word for it is {}-hay", input_word)
    } else {
        format!(
            "The Pig Latin word for it is {}-{}ay",
            rem_first(input_word),
            input_word.chars().next().unwrap()
        )
    }
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
