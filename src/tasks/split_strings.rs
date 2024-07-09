// https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/rust

pub fn split_strings(s: &str) -> Vec<String> {
   let mut result = s.split("").into_iter().enumerate().fold(Vec::new(), |mut acc, (index, x)| {
        if x == "" {
          return acc
        };

        let is_odd = index % 2 != 0;


        if is_odd {
            acc.push(x.to_string());
        } else {
            let count = acc.clone();
            acc[count.len() - 1].push_str(x);
        }

        acc
    });

    let is_odd = s.len() % 2 == 0;

    if !is_odd {
        let res_len = result.len();
        result[res_len - 1].push_str("_");
    }

    result
}

pub fn split_strings_2(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_string = String::new();

    for (index, c) in s.chars().enumerate() {
        if index % 2 == 0 {
            current_string.push(c);
        } else {
            current_string.push(c);
            result.push(current_string);
            current_string = String::new();
        }
    }

    if !s.is_empty() && s.len() % 2 != 0 {
        result.push(current_string + "_");
    }

    result
}