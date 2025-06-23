pub fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {

fn take_current_and_next(arr: &[u16], index: usize) -> &[u16] {
    let end = (index + 2).min(arr.len()); // index + 2, но не больше длины массива
    &arr[index..end] // Берём срез от index до end
}

    let mut result: u16 = 0;
    
    fn define_longest(pyramid: &[Vec<u16>], level: usize, slice: usize, intermediate: u16, new_result: &mut u16) {
        if pyramid.get(level) == None {
            return;
        }

        
        
        let current = pyramid[level].clone();

        let splited = take_current_and_next(&current, slice);
        
        for i in 0..splited.len() {
            let new_slice = intermediate + splited[i];
            let new_sl = slice + i;
            
            define_longest(pyramid, level + 1, new_sl, new_slice, new_result);
            
            if new_slice > *new_result {
                *new_result = new_slice;
            }
        }
        
    }



    define_longest(pyramid, 0, 0, 0, &mut result);

    println!("{:?}", result);

    result
}