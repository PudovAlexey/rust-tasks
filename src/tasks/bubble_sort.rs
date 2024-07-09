pub fn bubble_sort(mut arr: Vec<u64>) -> Vec<u64> {
    let len = arr.len();

    for i in 0..(len-1) {

        for j in 0..(len - 1) {
            if arr[i] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        };
    };

    println!("{:?}", arr);

    arr
}