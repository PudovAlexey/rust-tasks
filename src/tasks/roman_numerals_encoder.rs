use std::collections::HashMap;

// I          1
// V          5
// X          10
// L          50
// C          100
// D          500
// M          1,000

// 1224 - MCCXXIV

// IIII - IV
// VIIII - 9
// 

// MMMCMXCIX
// MMMDCCCCLXXXXVIV

pub fn roman_numerals_encoder(nums: i32) -> String {
    let mut number = nums;
    let mut roman = String::new();
    let numerals = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];

    for &(value, symbol) in numerals.iter() {
        while number >= value {
            roman.push_str(symbol);
            number -= value;
        }
    }

    roman
    
}