use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use crate::{get_calibration_value, get_calibration_value_sum, get_first_digit, get_last_digit, get_two_digit_number};

    #[test]
    fn test_calibration_value_sum() {
        assert_eq!(get_calibration_value_sum("src/example"), 142);
    }

    #[test]
    fn test_calibration_value() {
        assert_eq!(get_calibration_value("a1bc2".to_string()), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx".to_string()), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f".to_string()), 15);
        assert_eq!(get_calibration_value("treb7uchet".to_string()), 77);
    }

    #[test]
    fn test_two_digit_number() {
        assert_eq!(get_two_digit_number("a1bc2".to_string()), "12".to_string());
        assert_eq!(get_two_digit_number("pqr3stu8vwx".to_string()), "38".to_string());
        assert_eq!(get_two_digit_number("a1b2c3d4e5f".to_string()), "15".to_string());
        assert_eq!(get_two_digit_number("treb7uchet".to_string()), "77".to_string());
    }

    #[test]
    fn test_first_digit() {
        assert_eq!(get_first_digit("a1bc2".to_string()), "1".to_string());
        assert_eq!(get_first_digit("pqr3stu8vwx".to_string()), "3".to_string());
        assert_eq!(get_first_digit("a1b2c3d4e5f".to_string()), "1".to_string());
        assert_eq!(get_first_digit("treb7uchet".to_string()), "7".to_string());
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(get_last_digit("a1bc2".to_string()), "2".to_string());
        assert_eq!(get_last_digit("pqr3stu8vwx".to_string()), "8".to_string());
        assert_eq!(get_last_digit("a1b2c3d4e5f".to_string()), "5".to_string());
        assert_eq!(get_last_digit("treb7uchet".to_string()), "7".to_string());
    }
}

fn get_calibration_value_sum(path: &str) -> i32 {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .map(get_calibration_value)
        .sum()
}

fn get_calibration_value(line: String) -> i32 {
    let digits = get_two_digit_number(line);
    digits.parse().unwrap()
}

fn get_two_digit_number(line: String) -> String {
    let first = get_first_digit(line.clone());
    let last = get_last_digit(line.clone());
    first + &last   // but why &last?
}

fn get_first_digit(line: String) -> String {
    for c in line.chars() {
        if c.is_numeric() {
            return c.to_string();
        }
    }

    // Can I get rid of this? Crash would be ok in this case.
    return "".to_string();
}

fn get_last_digit(line: String) -> String {
    let line_reversed = line.chars().rev().collect();
    get_first_digit(line_reversed)
}


fn main() {
    let sum = get_calibration_value_sum("src/example");
    println!("{sum}")
}
