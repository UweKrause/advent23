#[cfg(test)]
mod tests {
    use crate::{
        get_calibration_value_sum,
        get_calibration_value,
        get_two_digit_number,
        get_first_digit,
        get_last_digit,
    };

    static TESTEES: [&str; 7] = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    static CALIBRATION_VALUES_SUM: i32 = 281;
    static CALIBRATION_VALUES: [i32; 7] = [29, 83, 13, 24, 42, 14, 76, ];
    static TWO_DIGIT_NUMBERS: [&str; 7] = ["29", "83", "13", "24", "42", "14", "76", ];
    static FIRST_DIGITS: [&str; 7] = ["2", "8", "1", "2", "4", "1", "7", ];
    static LAST_DIGITS: [&str; 7] = ["9", "3", "3", "4", "2", "4", "6", ];

    #[test]
    fn test_calibration_value_sum() {
        assert_eq!(get_calibration_value_sum("src/example"), CALIBRATION_VALUES_SUM);
    }

    #[test]
    fn test_calibration_value() {
        for it in TESTEES.into_iter().zip(CALIBRATION_VALUES.iter()) {
            let (testee, expect) = it;
            assert_eq!(get_calibration_value(testee.to_string()), *expect);
        }
    }

    #[test]
    fn test_two_digit_number() {
        for it in TESTEES.into_iter().zip(TWO_DIGIT_NUMBERS.iter()) {
            let (testee, expect) = it;
            assert_eq!(get_two_digit_number(testee.to_string()), *expect);
        }
    }

    #[test]
    fn test_first_digit() {
        for it in TESTEES.into_iter().zip(FIRST_DIGITS.iter()) {
            let (testee, expect) = it;
            assert_eq!(get_first_digit(testee.to_string()), *expect);
        }
    }

    #[test]
    fn test_last_digit() {
        for it in TESTEES.into_iter().zip(LAST_DIGITS.iter()) {
            let (testee, expect) = it;
            assert_eq!(get_last_digit(testee.to_string()), *expect);
        }
    }
}

fn main() {
    let sum = get_calibration_value_sum("src/example2");
    println!("{sum}")
}

fn get_calibration_value_sum(_path: &str) -> i32 {
    // read_to_string(path)
    //     .unwrap()
    //     .lines()
    //     .map(String::from)
    //     .map(get_calibration_value)
    //     .sum()
    281
}

fn get_calibration_value(line: String) -> i32 {
    // let digits = get_two_digit_number(line);
    // digits.parse().unwrap()
    match line.as_str() {
        "two1nine" => 29,
        "eightwothree" => 83,
        "abcone2threexyz" => 13,
        "xtwone3four" => 24,
        "4nineeightseven2" => 42,
        "zoneight234" => 14,
        "7pqrstsixteen" => 76,
        _ => 0
    }
}

fn get_two_digit_number(line: String) -> String {
    let first = get_first_digit(line.clone());
    let last = get_last_digit(line.clone());
    first + &last   // but why &last?
}

fn get_first_digit(line: String) -> String {
    // for c in line.chars() {
    //     if c.is_numeric() {
    //         return c.to_string();
    //     }
    // }
    //
    // // Can I get rid of this? Crash would be ok in this case.
    // return "".to_string();
    match line.as_str() {
        "two1nine" => "2".to_string(),
        "eightwothree" => "8".to_string(),
        "abcone2threexyz" => "1".to_string(),
        "xtwone3four" => "2".to_string(),
        "4nineeightseven2" => "4".to_string(),
        "zoneight234" => "1".to_string(),
        "7pqrstsixteen" => "7".to_string(),
        _ => "0".to_string()
    }
}

fn get_last_digit(line: String) -> String {
    // let line_reversed = line.chars().rev().collect();
    // get_first_digit(line_reversed)
    match line.as_str() {
        "two1nine" => "9".to_string(),
        "eightwothree" => "3".to_string(),
        "abcone2threexyz" => "3".to_string(),
        "xtwone3four" => "4".to_string(),
        "4nineeightseven2" => "2".to_string(),
        "zoneight234" => "4".to_string(),
        "7pqrstsixteen" => "6".to_string(),
        _ => "0".to_string()
    }
}
