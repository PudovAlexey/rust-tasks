// https://www.codewars.com/kata/5659c6d896bc135c4c00021e/train/rust


pub fn next_smaller_number(n: u64) -> Option<u64> {
    let mut result: Vec<u64> = Vec::new();

    let cl = n.clone();

    let perms: Vec<String> = n.to_string().chars().map(|c| c.to_string()).collect();

    result.push(n);

    for i in 0..perms.len() {
        let pointer = perms.get(i);
        let mut clone = perms.clone();

        // clone.splice(i, 1)
        clone.remove(i);

        for j in 0..perms.len() {
            let mut inside_clone = clone.clone();

            if i != j {
                inside_clone.insert(j, pointer.unwrap().to_owned());
            }

            let num: u64 = inside_clone.join("").parse().unwrap_or(0);
            let num_len: String = num.to_string().split("").collect();

            let index = result.iter().position(|&x| x == n); 


            if !index.is_none() && num_len.len() == perms.len() {
                result.push(num)
            };
        }
    };

    result.sort();

    let sorted: Vec<&u64> = result.iter().filter(|&el| el <= &n).collect();

   let s =  sorted.get(sorted.len() - 2);

   let opt_u64: Option<u64> = s.map(|&ref_ref| *ref_ref);

   println!("{:?}", opt_u64);

   None
}

// 123 
// 123 213 231 213 132
