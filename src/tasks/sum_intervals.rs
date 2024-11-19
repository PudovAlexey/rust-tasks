pub fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {

    let mut sorted_intervals: Vec<(i32, i32)> = intervals.to_vec();
    sorted_intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let merged_intervals = sorted_intervals.iter().fold(Vec::new(), |mut acc: Vec<(i32, i32)>, &(start, end)| {
        if let Some(&(last_start, last_end)) = acc.last() {
            if last_end >= start {
                let new_end = last_end.max(end);
                if let Some(last) = acc.last_mut() {
                    *last = (last_start, new_end);
                }
            } else {
                acc.push((start, end));
            }
        } else {
            acc.push((start, end));
        }
        acc
    });

    merged_intervals.iter().fold(0, |mut acc, &(a, b)| {
        acc += b - a;

      acc  
    })
}

                //1 ======================================== 4//

                              //3 ====================================== 10//

                                   //4 ======================= 6 //