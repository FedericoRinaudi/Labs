/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let mut sum: u8 = 0;
    let mut count = 0;
    for (i, c) in code.chars().rev().enumerate() {
        println!("{} {}", i, c);
        match c {
            c if c.is_digit(10) => {
                sum += (match count % 2 == 1 {
                    true => match c.to_digit(10).unwrap() {
                        9 => 9,
                        _ => (c.to_digit(10).unwrap() * 2) % 9,
                    },
                    false => c.to_digit(10).unwrap(),
                }) as u8;
                count += 1;
            }
            c if c == ' ' => {}
            _ => {
                return false;
            }
        }
    }
    return sum % 10 == 0 && count > 1;
}
