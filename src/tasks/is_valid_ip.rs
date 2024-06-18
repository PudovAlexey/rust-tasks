// 1.2.3.4

pub fn is_valid_ip(ip: &str) -> bool {

    let split_chars: Vec<&str> = ip.split(".").collect();

    match split_chars.len() {
        4 => {
            let mut is_valid = true;

            for char in split_chars {
                let numeric: i32 = char.parse().unwrap_or(-1);

                if !(numeric >= 0 && numeric <= 255) {
                    is_valid = false;
                    break;
                }
            }

            is_valid
        },
        _ => false
    }

}