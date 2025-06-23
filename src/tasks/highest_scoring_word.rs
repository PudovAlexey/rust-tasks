use std::collections::HashMap;

pub fn highest_scoring_word(input: &str) -> &str {
let mut top_matcher = 0;
let mut max_word = "";
let alphabet = "abcdefghijklmnopqrstuvwxyz";
let split_world: Vec<&str> = input.split(" ").collect();
let alphabet_vec: Vec<&str> = alphabet.split("").collect();

    for word in split_world {
        let mut word_matcher = 0;

        for letter in word.chars() {
            let index = alphabet_vec.iter().position(|&e| e == letter.to_string());

            word_matcher += index.unwrap()
        }

        if top_matcher < word_matcher {
            max_word = word;
            top_matcher  = word_matcher
        }
    }

    println!("{}", max_word);

 max_word
}

pub struct WordsWithScore {
    pub word: String,
    pub score: i32,
}
