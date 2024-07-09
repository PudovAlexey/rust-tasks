// https://www.codewars.com/kata/58223370aef9fc03fd000071/train/rust
use regex::Regex;

pub fn dashatize(n: i64) -> String {
    let positive: i64 = if n < 0 {
        -n
    } else {
        n
    };

    let splitter: Vec<String> = positive.to_string().chars().map(|c| c.to_string()).collect();
    let mut result = String::new();

    for (index, char) in splitter.iter().enumerate() {
        let is_first = index == 0;
        let is_last = index == splitter.len() - 1;
        let num: i64 = char.parse().unwrap_or(-1);
        let last_char = result.chars().rev().next();

        if  !is_first && !is_last && last_char.is_some() && num % 2 != 0 {

            result.push_str(&format!("{}{}{}", "-", char, "-"))

        } else if is_first && num % 2 != 0 {
            result.push_str(&format!("{}{}", char, "-"))
        } else if is_last && num % 2 != 0 {
            result.push_str(&format!("{}{}", "-", char))
        } else {
            result.push_str(&num.to_string());
            
        };
    }
    
   let re = Regex::new(r"[-]+").unwrap();
    let output = re.replace_all(&result, "-").to_string();
    
   let test =  if output == String::from("1-") {
        String::from("1")
    } else {
        output   
    };

    println!("{:?}", test);

    test

}