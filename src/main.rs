use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = String; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut to_vector = s
            .chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap_or_default() as u8)
            .collect::<Vec<u8>>();
        if to_vector.len() != 13 {
            Err("Error: Invalid ISBN Size".to_string())
        } else {
            let check_digit = to_vector.pop().unwrap();
            if calculate_check_digit(&to_vector) == check_digit {
            Ok(Isbn {
            raw: s.to_owned(),
            digits: to_vector,
            })
        } else {
            Err("Error: Invalid ISBN Checksum".to_string())
        }}
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut check_bit: u8 = 0;
    let mut checksum_value: u32 = 0;
    for digit in digits {
        checksum_value += match check_bit {
            0 => { check_bit = 1; *digit as u32 }
            1 => { check_bit = 0; (*digit as u32) * 3 }
            _ => { check_bit = 0; 0 }
        }
    }
    match checksum_value % 10 {
        0 => 0,
        _ => (10 - checksum_value % 10) as u8,
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({}) is valid!", rust_in_action.raw);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let isbn: Isbn = "978-3-16-148410-0".parse().unwrap();
    let _ = isbn.digits;
}
