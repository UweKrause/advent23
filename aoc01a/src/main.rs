use std::fs::read_to_string;

#[cfg(test)]
mod calibration_value {
    #[test]
    fn get_calibration_value_sum() {
        assert_eq!(crate::get_calibration_value_sum("src/example"), 142);
    }

    #[test]
    fn get_calibration_value() {
        assert_eq!(crate::get_calibration_value("a1bc2"), 12);
        assert_eq!(crate::get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(crate::get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(crate::get_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn get_two_digit_number() {
        assert_eq!(crate::get_two_digit_number("a1bc2"), "12");
        assert_eq!(crate::get_two_digit_number("pqr3stu8vwx"), "38");
        assert_eq!(crate::get_two_digit_number("a1b2c3d4e5f"), "15");
        assert_eq!(crate::get_two_digit_number("treb7uchet"), "77");
    }

    #[test]
    fn get_first_digit() {
        assert_eq!(crate::get_first_digit("a1bc2"), '1');
        assert_eq!(crate::get_first_digit("pqr3stu8vwx"), '3');
        assert_eq!(crate::get_first_digit("a1b2c3d4e5f"), '1');
        assert_eq!(crate::get_first_digit("treb7uchet"), '7');
    }

    #[test]
    fn get_last_digit() {
        assert_eq!(crate::get_last_digit("a1bc2"), '2');
        assert_eq!(crate::get_last_digit("pqr3stu8vwx"), '8');
        assert_eq!(crate::get_last_digit("a1b2c3d4e5f"), '5');
        assert_eq!(crate::get_last_digit("treb7uchet"), '7');
    }
}

fn main() {
    let sum = get_calibration_value_sum("src/input");
    println!("{sum}") // 55130
}

fn get_calibration_value_sum(path: &str) -> i32 {
    read_to_string(path).unwrap()
        .lines()
        .map(get_calibration_value)
        .sum()
}

fn get_calibration_value(line: &str) -> i32 {
    let digits = get_two_digit_number(line);
    digits.parse().unwrap()
}

fn get_two_digit_number(line: &str) -> String {
    let mut ret = String::new();
    ret.push(get_first_digit(line));
    ret.push(get_last_digit(line));
    return ret;
}

fn get_first_digit(line: &str) -> char {
    let mut ret: char = 'a';
    for c in line.chars() {
        if c.is_numeric() {
            ret = c;
            break;
        }
    }
    ret
}

fn get_last_digit(line: &str) -> char {
    let line_reversed: String = line.chars().rev().collect();
    get_first_digit(&line_reversed)
}
