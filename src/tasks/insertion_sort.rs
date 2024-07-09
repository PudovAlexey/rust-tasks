// 4
// 1 10 15 20

pub fn insertion_sort(arr: Vec<u64>) -> Vec<u64> {
    let mut sorted: Vec<u64> = Vec::new();

    sorted.push(arr[0]);

    for &el in arr[1..].iter() {

        
        for idx in (0..sorted.len()).rev() {
            let sort_el = sorted[idx];

            if sort_el > el {
                continue;
            }
            
            if idx == sorted.len() - 1 {
                sorted.push(el);
                break;
            } else {
            sorted.insert(idx + 1, el);
            break;
        }     

        }
    }

    sorted
}