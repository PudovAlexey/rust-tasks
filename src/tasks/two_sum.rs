use std::collections::HashMap;

// my test
// fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
//     let mut result: (usize, usize) = (0, 0);

//     for (i, el) in  numbers.into_iter().enumerate() {
        
//         for (j, in_el) in numbers.into_iter().enumerate() {
//          if (el + in_el == target && i != j) {
//             result = (i, j)
//          }   
//         }
//     }

//     result
// }

pub fn two_sum(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(numbers.len());

    for (i, &v) in numbers.iter().enumerate() {
        match map.get(&(target - v)) {
            Some(&idx) => return Some((i, idx)),
            None => {
                map.insert(v, i);
                continue;
            },
        }
    }


    println!("{:?}", map);

    unreachable!();
}