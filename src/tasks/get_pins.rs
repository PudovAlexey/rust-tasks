use std::collections::HashSet;

pub fn get_pins(observed: &str) -> Vec<String> {
    let matrix_nums = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [-1, 0, -1],
    ];

    // Проверка на выход за границы
    fn define_is_out_of_bounds(matrix_nums: &[[i32; 3]; 4], x: i32, y: i32) -> Option<i32> {
        if x < 0 || y < 0 {
            return None;
        }

        if let Some(row) = matrix_nums.get(x as usize) {
            if let Some(&value) = row.get(y as usize) {
                if value >= 0 {
                    return Some(value);
                }
            }
        }
        None
    }

    // Получение соседних значений
    fn define_dfs_sum(x: i32, y: i32, matrix_nums: &[[i32; 3]; 4]) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for i in [
            define_is_out_of_bounds(matrix_nums, x, y),
            define_is_out_of_bounds(matrix_nums, x, y - 1),
            define_is_out_of_bounds(matrix_nums, x, y + 1),
            define_is_out_of_bounds(matrix_nums, x - 1, y),
            define_is_out_of_bounds(matrix_nums, x + 1, y),
        ] {
            if let Some(el) = i {
                result.push(el);
            }
        }
        result
    }

    // Получение соседних значений для каждой цифры в observed
    fn split_observed_values(observed: &str, matrix_nums: &[[i32; 3]; 4]) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for i in 0..matrix_nums.len() {
            let matrix_incl = matrix_nums[i].len();
            for j in 0..matrix_incl {
                let &n = matrix_nums[i].get(j).unwrap();
                if n.to_string() == observed {
                    result = define_dfs_sum(i as i32, j as i32, matrix_nums);
                    break;
                }
            }
        }
        result
    }

    // Генерация всех возможных комбинаций
    fn generate_combinations(
        combinations: &Vec<Vec<i32>>,
        index: usize,
        current: &mut Vec<i32>,
        results: &mut HashSet<String>,
    ) {
        if index == combinations.len() {
            // println!("{} {:?}", index, current);
            results.insert(current.iter().map(|&x| x.to_string()).collect());
            return;
        }
        
        for &num in &combinations[index] {
            current.push(num);
            println!("{:?} before", current);
            generate_combinations(combinations, index + 1, current, results);
            println!("{:?} after", current);
            current.pop();
        }
    }

    let mut combinations: Vec<Vec<i32>> = Vec::new();

    for i in observed.chars() {
        let str = i.to_string();
        let res = split_observed_values(&str, &matrix_nums);
        combinations.push(res);
    }

    let mut results: HashSet<String> = HashSet::new();
    let mut current: Vec<i32> = Vec::new();
    println!("{:?}", combinations);
    generate_combinations(&combinations, 0, &mut current, &mut results);

    // Преобразуем HashSet в Vec<String> для возврата
    results.into_iter().collect()
}