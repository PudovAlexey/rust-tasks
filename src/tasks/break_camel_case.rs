use regex::Regex;

pub fn break_camel_case(s: &str) -> String {
    let new_word = String::new();
    s.chars().into_iter().fold(String::new(), |mut acc, el| {
        let upper_char = el.to_ascii_uppercase();

        if upper_char == el {
            acc.push_str(" ");
            acc.push_str(&el.to_string());
        } else {
            acc.push_str(&el.to_string());
        }
        
        acc
    });
    // let regex_camels = "/D/";

    new_word
}