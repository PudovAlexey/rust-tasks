fn last_digit(nums: &[u64]) -> u32 {
    let a_last_digit = nums[0] % 10; // Последняя цифра первого числа

    // Если в массиве только одно число (a^0), то возвращаем 1
    if nums.len() == 1 {
        return 1;
    }

    // Объединяем остальные числа в одно число b
    let b = nums[1..].iter().fold(0, |acc, &x| acc * 10 + x);

    // Если b равно 0, a^0 = 1
    if b == 0 {
        return 1;
    }

    // Период для последних цифр
    let period = match a_last_digit {
        0 => 1,
        1 => 1,
        2 => 4,
        3 => 4,
        4 => 2,
        5 => 1,
        6 => 1,
        7 => 4,
        8 => 4,
        9 => 2,
        _ => 1,
    };

    // Находим индекс в периоде
    let b_mod = if b % period == 0 { period } else { b % period };

    // Возвращаем последнюю цифру
    let last_digit_result = match a_last_digit {
        0 => 0,
        1 => 1,
        2 => [6, 2, 4, 8][(b_mod - 1) as usize],
        3 => [1, 3, 9, 7][(b_mod - 1) as usize],
        4 => [6, 4][(b_mod - 1) as usize],
        5 => 5,
        6 => 6,
        7 => [1, 7, 9, 3][(b_mod - 1) as usize],
        8 => [6, 8, 4, 2][(b_mod - 1) as usize],
        9 => [1, 9][(b_mod - 1) as usize],
        _ => 0,
    };

    last_digit_result
}