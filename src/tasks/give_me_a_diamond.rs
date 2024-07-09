// https://www.codewars.com/kata/5503013e34137eeeaa001648/train/rust
// *
// ***
// *****
// ***

// 3 - 1
// 5 - 
pub fn give_me_a_diamond(n: i32) -> Option<String> {
    let mut count: i32 = -1;
    
    let is_even = n % 2 == 0;
    
    if n <= 0 || is_even {
        return None;
    }
    
    let mut result = String::new();
    let middle = n / 2; 
    let mut pass_spacess = middle + 1;

    if n == 1 {
       return Some("*\n".to_string());
    }

    for i in 0..n {

        if i <= middle {
            count += 2;
            pass_spacess -=1;
        } else {
            count -= 2;
            pass_spacess +=1;
        }
        
        
        let spaces = " ".repeat(pass_spacess as usize);
        result.push_str(&spaces);

        let stars = "*".repeat(count as usize);

        result.push_str(&stars);

        result.push_str("\n")

    }

 Some(result)
}