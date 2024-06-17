pub fn search_substr(full_text: &str, search_text: &str, allow_overlap: bool) -> i32 {
    let mut values: Vec<(i32, i32)> = Vec::new();
    let mut result = 0;

    for index in 0..full_text.len() {
        let mut is_overlap = false;
        let start_range = index;
        let end_range = index + search_text.len();
        let typed_start_range = index as i32;
        let typed_end_range = (index + search_text.len()) as i32;
        let slice = if full_text.len() >= (index + search_text.len()) {
            &full_text[index..(index + search_text.len())]
        } else {
            ""
        };

        if !allow_overlap {
            is_overlap = values.iter().find(|el| {

            if typed_start_range <= el.0 && typed_end_range >= el.1 {
             true
            } else if typed_start_range >= el.0 && typed_end_range <= el.1 {
             true
            } else if typed_start_range <= el.1 && typed_end_range >= el.1 {
                true
            } else if typed_start_range <= el.0 && typed_end_range >= el.0 {
                true
            } else {
                false
            }

            }).is_some();
        }

        if slice == search_text && !is_overlap {
            result += 1;
            values.push((index as i32, (index + search_text.len()) as i32))
        }
    }

    println!("{}", result);

    result
}

//             |------------------------|
//      |---------------|

//              |--------------|

//                  |-------|
//               |-------------|