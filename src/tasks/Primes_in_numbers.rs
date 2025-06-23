pub fn primes_in_numbers(mut n: i64) -> String {
    let mut multiplier = 2;

    let mut resulted_string = String::new();

    while n > 1 {
        let is_allow_multiply = n % multiplier == 0;

        println!("{}", is_allow_multiply);

        if is_allow_multiply {
            let check_for_more_than_once = (n / multiplier) % multiplier == 0;

            if check_for_more_than_once {
                let mut multiplies_times = 0;
                loop {
                    let is_allow_to_sep = n % multiplier == 0;

                    if is_allow_to_sep {
                        n = n / multiplier;
                        multiplies_times += 1;
                    } else {
                        break;

                    }

                }

                resulted_string.push_str(format!("({}**{})", multiplier, multiplies_times).as_str());
            } else {
                n = n / multiplier;

                resulted_string.push_str(format!("({})", multiplier).as_str());
            }
        }

        multiplier +=1
    }

        resulted_string
        
    }