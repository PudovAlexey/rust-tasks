pub fn determinant(matrix: &[Vec<i64>]) -> i64 {
    let mut result = 0;
    if matrix.len() == 1 {
        let mt = matrix.get(0);

        let one = mt.unwrap().get(1).unwrap();

        return one.to_owned();
    }

    let is_base = matrix.len() == 2;

    if is_base {
        let left_side = matrix
        .get(0)
        .unwrap()
        .get(0)
        .unwrap() * matrix.get(1)
        .unwrap()
        .get(1).unwrap();

        let right_side = matrix
        .get(0)
        .unwrap()
        .get(1)
        .unwrap() * matrix
        .get(1)
        .unwrap()
        .get(0)
        .unwrap();

        return left_side - right_side;
    } else {

        let first_row = matrix.get(0).unwrap();
        
        // let determine_matrix = Vec::new();
        
        for it in 0..first_row.len() {

            let first_row_data = first_row.get(it)
            .unwrap();

            let mut new_matrix: Vec<Vec<i64>> = Vec::new();

        for row in 0..matrix.len() {
            let matrix_row = matrix.get(row).unwrap();
            let mut new_row: Vec<i64> = Vec::new();
            for col in 0..matrix_row.len() {
               if col != it && row != 0 {
                let val = matrix
                .get(row)
                .unwrap()
                .get(col)
                .unwrap();

                new_row.push(*val); 
               } 
            }

            if !new_row.is_empty() {
                new_matrix.push(new_row);
            }
        }
        let dataa_pass = if it % 2 == 0 {
            first_row_data
        } else {
            &-first_row_data
        };

        result += dataa_pass * determinant(&new_matrix);

        }
    }

    result
}
