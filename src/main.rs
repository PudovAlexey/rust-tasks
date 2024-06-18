pub mod tasks;

use tasks::{
   // find_all_anagrams_in_string,
   highest_scoring_word, make_linked_list, roman_numerals_encoder, search_substr::{self, search_substr}, two_sum, Merge_k_sorted_lists,
   is_valid_ip
};

fn main() {
   // let search = search_substr("aabbaaaaaaaa", "aaa", false);

   // (4, 5), (5,6), (6, 7) 
   let is_valid =  is_valid_ip::is_valid_ip("-1.3.zhopa.3.4");

   println!("{}",  is_valid)


}