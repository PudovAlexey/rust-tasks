pub fn perimeter_of_squares_rectangle(n: u64) -> u64 {
    // your code

    let mut fib_counter: Vec<u64> = Vec::from([0, 1]);

    for i in 1..n + 1 {

        fib_counter.push(
            fib_counter.get( i as usize - 1).unwrap_or(&0) + fib_counter.get(i as usize).unwrap_or(&0)
        );
    }

    let result = 4 * fib_counter.iter().fold(0 as u64, | mut acc, &item| {
        acc += item;

        acc
    });

    result
  }