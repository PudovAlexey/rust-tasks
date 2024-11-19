pub fn zeros(n: u64) -> u64 {
    let mut count = 0;
    let mut divisor = 5;

    while n >= divisor {
        count += n / divisor;
        divisor *= 5;
    }

    count
}

pub fn zeros2(n: u64) -> u64 {
    let mut summary = 1;
    for ch in 1..=n {
        summary *= ch;
    }

    let summary_str = summary.to_string();
    let trailing_zeros = summary_str.chars().rev().take_while(|&c| c == '0').count();
    
    trailing_zeros as u64
}
