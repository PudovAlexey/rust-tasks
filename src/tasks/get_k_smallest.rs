use std::cmp::Ordering;

pub fn get_k_smallest<T: Copy + Ord + PartialOrd>(arr: &mut [T], k: usize) -> Vec<T> {
    if arr.len() == 0 || k == 0 {
        return vec![];
    }

    let pivot = partition(arr);
    let c = pivot + 1;
    match c.cmp(&k) {
        Ordering::Less => {
            let mut res = arr[0..c].to_vec();
            res.append(&mut get_k_smallest(&mut arr[c..], k - c));
            res
        }
        Ordering::Greater => get_k_smallest(&mut arr[0..pivot].to_vec(), k),
        Ordering::Equal => arr[0..c].to_vec()
    }
}

fn partition<T: Copy + Ord + PartialOrd>(arr: &mut [T]) -> usize {
    let length = arr.len();
    let pivot_val = arr[length - 1];
    let mut store_index = 0;

    for i in 0..arr.len() - 1 {
        if arr[i] < pivot_val {
            arr.swap(store_index, i);
            store_index += 1;
        }
    }

    arr.swap(store_index, length - 1);
    store_index
}
