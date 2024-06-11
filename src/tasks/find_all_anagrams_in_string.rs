fn sort(chars: &str) -> String {
  let mut chars = chars.chars().collect::<Vec<char>>();
  chars.sort_by_key(|k| k.to_ascii_lowercase());
  chars.into_iter().collect()
}

pub fn find_all_anagrams_in_string(str: &str, chars: &str) -> Vec<usize> {
  let sorted_chars = sort(chars);
  let mut result = Vec::new();

  for (i, _) in str.chars().enumerate() {
      if i < str.len() - sorted_chars.len() + 1 {
          let substring = &str[i..i + sorted_chars.len()];
          let sorted_substring = sort(substring);

          if sorted_substring == sorted_chars {
              result.push(i);
          }
      }
  }

  result
}