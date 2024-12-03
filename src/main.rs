pub mod tasks;

use tasks::{find_erimp, get_pins, hamming_numbers::hamming, last_digit, matrix_determinant::determinant, perimeter_of_squares_rectangle, sudoku_encoder, sum_intervals};

fn main() {

   // let result = find_erimp::find_erimp(50 as u32);

   // println!("{:?}", result);
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

   // let resule = sum_intervals::sum_intervals(&[(1, 4), (7, 10), (3, 5)]);

   // println!("{:?}", resule);

   // let get_pin_combinations = get_pins::get_pins("369");

   // println!("combo {:?}", get_pin_combinations)

   // let det = determinant(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]]);

   // let did = last_digit(vec![3, 4, 5]);
   // let param: Vec<u64> = vec![1, 2];

   // let test = last_digit::last_digit(&param);

let mut puzzle: [[u8; 9]; 9] = [
    [5, 3, 4, 6, 7, 8, 9, 1, 2],
    [6, 7, 2, 1, 9, 5, 3, 4, 8],
    [1, 9, 8, 3, 4, 2, 5, 6, 7],
    [8, 5, 9, 7, 6, 1, 4, 2, 3],
    [4, 2, 6, 8, 5, 3, 7, 9, 1],
    [7, 1, 3, 9, 2, 4, 8, 5, 6],
    [9, 6, 1, 5, 3, 7, 2, 8, 4],
    [2, 8, 7, 4, 1, 9, 6, 3, 5],
    [3, 4, 5, 2, 8, 6, 1, 7, 9],
];

    // let result = sudoku_encoder::sudoku(&mut puzzle);

    // println!("{:?}", result);

   // println!("{}", test);

   let result = hamming(19);


}

// let mtr = [
//   [2, 5, 3],
//   [1, -2, -1],
//   [1, 3, 4],
// ]