//https://www.codewars.com/kata/526d84b98f428f14a60008da/train/rust

pub fn hamming(n: usize) -> u64 {
    let mut hamming_nums = vec![1];
    let (mut i2, mut i3, mut i5) = (0, 0, 0);

    while hamming_nums.len() < n {
        let next2 = hamming_nums[i2] * 2;
        let next3 = hamming_nums[i3] * 3;
        let next5 = hamming_nums[i5] * 5;

        let next_hamming = next2.min(next3).min(next5);
        hamming_nums.push(next_hamming);

        if next_hamming == next2 {
            i2 += 1;
        }
        if next_hamming == next3 {
            i3 += 1;
        }
        if next_hamming == next5 {
            i5 += 1;
        }
    }

    hamming_nums[n - 1]
}
