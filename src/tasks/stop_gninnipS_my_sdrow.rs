pub fn spin_words(words: &str) -> String {
    let words: Vec<&str> = words.split(" ").collect();

    words.iter().enumerate().fold( String::new(), |mut acc, (index, &word)| {

        if (index >0) {
            acc.push_str(" ");
        }

        if (word.len() >= 5) {
            let reverse: String = word.chars().rev().collect();

            acc.push_str(&reverse)
        } else {
            acc.push_str(word);
        }

        acc
    })
}