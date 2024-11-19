// pub fn find_erimp(n: u32) -> (u32, u32, u64) {
//  let mut range: Vec<u32> = Vec::new();


 
//  for item in 2..n {
//         range.push(item);
    
//  }
//  fn define_range(pointer: usize, filtered: Vec<u32>, f: u32) -> Vec<u32> {
//     let simple_num = filtered.get(pointer);

//     match simple_num {
//         Some(simple) => {

//             if simple * simple >= f {
//                 return filtered;
//             }
            
//                 let mut filter_range: Vec<u32> = filtered.iter().filter(|&&num| {

//                     num % simple != 0
//                 })
//                 .map(|&el| el).collect();
            
//                 filter_range.insert(pointer, simple.to_owned());
//                 define_range(pointer + 1, filter_range, f)

//         },
//         None => filtered
//     }

//  }

//  let simpe_range =  define_range(0, range, n);

//  let filter_simple_range: Vec<&u32> = simpe_range.iter().filter(|&&e| {
//     let digits: Vec<char> = e.to_string().chars().collect();
//     let sorted_string: String = digits.into_iter().rev().collect();
//     let num_sort = sorted_string.parse::<u32>().unwrap();

//     let mut range: Vec<u32> = Vec::new();


 
//     for item in 2..num_sort + 10 {
//            range.push(item);
       
//     }

//     let define_simple = define_range(0, range, num_sort + 10);

//     let find_target_erimp = define_simple.iter().find(|&&e| {
//         e == num_sort
//     });

//     e != num_sort && find_target_erimp.is_some()
//  }).collect();

// let mut sum: u64 = 0;
// let mut  largest: u32 = 0;
// let mut length: u32 = 0;
// for &i in filter_simple_range {
//     length += 1;
//     sum += i as u64;
    
//     if largest < i {
//         largest = i
//     }
// };

// (length, largest, sum)
// }

pub fn find_erimp(n: u32) -> (u32, u32, u64) {
    // Используем вектор для определения, является ли число простым
    let mut is_prime = vec![true; n as usize];
    let mut primes = Vec::new();

    for i in 2..n as usize {
        if is_prime[i] {
            primes.push(i as u32);
            let mut multiple = i * i;
            while multiple < n as usize {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    // Функция для переворачивания числа
    fn reverse_number(mut num: u32) -> u32 {
        let mut reversed = 0;
        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        reversed
    }

    let mut sum: u64 = 0;
    let mut largest: u32 = 0;
    let mut length: u32 = 0;

    for &prime in &primes {
        let reversed = reverse_number(prime);
        if reversed != prime && reversed < n && is_prime[reversed as usize] {
            length += 1;
            sum += prime as u64;
            if largest < prime {
                largest = prime;
            }
        }
    }

    (length, largest, sum)
}