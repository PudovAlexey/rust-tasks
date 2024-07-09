pub mod tasks;

use tasks::{
   // find_all_anagrams_in_string,
   give_me_a_diamond::{self, give_me_a_diamond}, highest_scoring_word, is_valid_ip, make_linked_list, roman_numerals_encoder, search_substr::{self, search_substr}, two_sum, 
   Merge_k_sorted_lists,
   Next_smaller_number_with_the_same_digits,
   dashatizeOdd,
   bubble_sort,
   insertion_sort,
   quick_sort,
   // ips_between
   split_strings,
};

fn main() {
   // let search = search_substr("aabbaaaaaaaa", "aaa", false);

   // (4, 5), (5,6), (6, 7) 
   // let is_valid =  is_valid_ip::is_valid_ip("-1.3.zhopa.3.4");

   // println!("{}",  is_valid)

   // let diamond = give_me_a_diamond(5);
   // Next_smaller_number_with_the_same_digits::next_smaller_number(531);
   // dashatizeOdd::dashatize(-338896294);
   // let vec: Vec<u64> = Vec::from([1, 4,  7, 2, 5]);

   // quick_sort::quick_sort(vec);

   // ips_between::ips_between("170.0.0.0", "170.1.0.0");

   let test = split_strings::split_strings("abcdefgt");

   println!("{:?}", test);


}