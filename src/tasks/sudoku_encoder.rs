//https://habr.com/ru/articles/173795/

// https://www.codewars.com/kata/5296bc77afba8baa690002d7/rust
// use std::coll
use std::collections::HashMap;


pub fn sudoku(puzzle: &mut [[u8; 9]; 9]) -> [[u8; 9]; 9] {

    enum ValidationVariants {
        Valid(bool),
        Invalid(u8)
    }
    fn check_valid_row(row: &Vec<u8>, trying: Vec<u8>) -> ValidationVariants {
        let mut valid_config: HashMap<u8, bool> = HashMap::from([
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, true),
            (7, true),
            (8, true),
            (9, true),
        ]);
        
        for i in trying {
            valid_config.remove(&i);
        }
        let checker =  row.iter().all(|&x| {
            match valid_config.get(&x) {
                Some(_) => {
                    valid_config.remove(&x);
                    true
                },
                None => {
                    false
                }
            }
        });

        let num: Vec<u8> = valid_config.keys().cloned().collect();

        match checker {
            true => {
                ValidationVariants::Valid(true)
            },
            false => {
                ValidationVariants::Invalid(num[0])
            }
        }
    }

    
    fn is_valid_sudoku(puzzle: [[u8; 9]; 9], ) -> bool {
        let mut is_valid_base = true;
        let mut is_square_valid = true;
        
        for i in 0..puzzle.len() {
            let mut vertical_raw: Vec<u8> = Vec::new();
            let horizontal_raw: Vec<u8> = puzzle[i].into();
            let mut next_string_index: usize = 0;

            while puzzle[i].get(next_string_index).is_some() {
                let value = puzzle[i].get(next_string_index);

                if let Some(&ex_val) = value {
                    vertical_raw.push(ex_val);

                }
                next_string_index +=1;
            }

            let check_vertical = check_valid_row(&vertical_raw, Vec::new());
            let check_horizontal = check_valid_row(&horizontal_raw, Vec::new());

            if let ValidationVariants::Invalid(_) = check_vertical {
                is_valid_base = false;
                break;
            }

            if let ValidationVariants::Invalid(_) = check_horizontal {
                is_valid_base = false;
                break;
            }
        }

        for row_steps in (0..puzzle.len()).step_by(3) {
            for column_steps in (0..puzzle[row_steps].len()).step_by(3) {
                let mut valid_square: Vec<u8> = Vec::new();

                for i in row_steps..row_steps + 3 {
                    for j in column_steps..column_steps + 3 {
                       valid_square.push(puzzle[i][j]); 
                        
                    }
                } 

                if let ValidationVariants::Invalid(_) = check_valid_row(&valid_square, Vec::new()) {
                    is_square_valid  = false;
                    break;
                }
            }
        }

        is_valid_base & is_square_valid
    }

    // println!("{}", is_valid_sudoku);

    fn try_fill_and_check(value: &mut [[u8; 9]; 9]) -> [[u8; 9]; 9] {
        let is_valid = is_valid_sudoku(*value);

        if is_valid {
            *value
        } else {
            let mut result = value.clone();

            for i in 0..value.len() {
                for j in 0..value.len() {
                    if value[i][j] == 0 {
                        
                        for i in 1..9 {
                            result[i][j] = i as u8;
                            result = try_fill_and_check(&mut result);
                        }
                    }
                }
            }

            result
        }

    }
    try_fill_and_check(puzzle)
}

pub fn sudokuv2(puzzle: &mut [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    fn is_valid(puzzle: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
        for x in 0..9 {
            if puzzle[row][x] == num || puzzle[x][col] == num {
                return false;
            }
        }
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if puzzle[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }
        true
    }

    fn solve(puzzle: &mut [[u8; 9]; 9]) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if puzzle[row][col] == 0 {
                    for num in 1..=9 {
                        if is_valid(puzzle, row, col, num) {
                            puzzle[row][col] = num;
                            if solve(puzzle) {
                                return true;
                            }
                            puzzle[row][col] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    solve(puzzle);
    *puzzle
}