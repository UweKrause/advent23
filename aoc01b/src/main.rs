use std::cmp::{max, min};
use std::fs::read_to_string;

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
        assert_eq!(get_calibration_value_sum("src/example2"), CALIBRATION_VALUES_SUM);
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
    let sum = get_calibration_value_sum("src/input");
    println!("{sum}")
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

static NUMBERS: [(&str, &str); 9] = [
    // (number, textual representation),
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine"),
];

fn get_first_digit(line: String) -> String {
    // for every possible number (or its textual representation) check the first occurrence of it.
    // if this first occurrence is the smallest known occurrence, remember it and the number

    let mut smallest_pos = usize::MAX;
    let mut smallest_number = "";

    for (number, number_text) in NUMBERS {
        let pos = min(
            line.find(number).unwrap_or(usize::MAX),
            line.find(number_text).unwrap_or(usize::MAX),
        );

        if pos < smallest_pos {
            smallest_pos = pos;
            smallest_number = number;
        }
    }

    smallest_number.to_string()
}

fn get_last_digit(line: String) -> String {
    // ERROR IS HERE: Should be -1
    // fails for: "7dvt"
    let mut greatest_pos = 0;
    let mut greatest_number = "";

    for (number, number_text) in NUMBERS {
        let pos = max(
            line.rfind(number).unwrap_or(0),
            line.rfind(number_text).unwrap_or(0),
        );

        if pos > greatest_pos {
            greatest_pos = pos;
            greatest_number = number;
        }
    }

    greatest_number.to_string()
}
