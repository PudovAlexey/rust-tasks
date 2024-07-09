use rand::Rng;

pub fn quick_sort(items: Vec<u64>) -> Vec<u64> {
    fn recursive_sort(items: Vec<u64>) -> Vec<u64> {
        if items.len() <= 1 {
            return items;
        }

        let pivot_index = rand::thread_rng().gen_range(0..items.len());
        let pivot = items[pivot_index];

        let mut less = vec![];
        let mut equal = vec![];
        let mut greater = vec![];

        for i in 0..items.len() {
            if items[i] < pivot {
                less.push(items[i]);
            } else if items[i] == pivot {
                equal.push(items[i]);
            } else {
                greater.push(items[i]);
            }
        }

        let mut sorted_less = recursive_sort(less);
        let mut sorted_greater = recursive_sort(greater);

        sorted_less.extend(equal);
        sorted_less.extend(sorted_greater);
        sorted_less
    }

    let res = recursive_sort(items);

    println!("{:?}", res);

    res
}