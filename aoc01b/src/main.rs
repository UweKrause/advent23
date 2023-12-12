use std::cmp::{max, min};
use std::fs::read_to_string;

#[cfg(test)]
mod calibration_value {
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
    static FIRST_DIGITS: [char; 7] = ['2', '8', '1', '2', '4', '1', '7', ];
    static LAST_DIGITS: [char; 7] = ['9', '3', '3', '4', '2', '4', '6', ];

    #[test]
    fn get_calibration_value_sum() {
        assert_eq!(crate::get_calibration_value_sum("src/example2"), CALIBRATION_VALUES_SUM);
    }

    #[test]
    fn get_calibration_value() {
        for (testee, expect) in TESTEES.into_iter().zip(CALIBRATION_VALUES.iter()) {
            assert_eq!(crate::get_calibration_value(testee), *expect);
        }
    }

    #[test]
    fn get_two_digit_number() {
        for (testee, expect) in TESTEES.into_iter().zip(TWO_DIGIT_NUMBERS.iter()) {
            assert_eq!(crate::get_two_digit_number(testee), *expect);
        }
    }

    #[test]
    fn get_first_digit() {
        for (testee, expect) in TESTEES.into_iter().zip(FIRST_DIGITS.iter()) {
            assert_eq!(crate::get_first_digit(testee), *expect);
        }
    }

    #[test]
    fn get_last_digit() {
        for (testee, expect) in TESTEES.into_iter().zip(LAST_DIGITS.iter()) {
            assert_eq!(crate::get_last_digit(testee), *expect);
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

    ret
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

fn get_first_digit(line: &str) -> char {
    // for every possible number (or its textual representation)
    // check the first occurrence of it.
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

    smallest_number.parse().unwrap()
}

fn get_last_digit(line: &str) -> char {
    let mut greatest_pos = -1;
    let mut greatest_number = "";

    for (number, number_text) in NUMBERS {
        let pos_number = line.rfind(number);
        let pos_number_text = line.rfind(number_text);

        if pos_number.is_some() | pos_number_text.is_some() {
            let pos = max(pos_number, pos_number_text).unwrap() as i32;

            if pos > greatest_pos {
                greatest_pos = pos;
                greatest_number = number;
            }
        }
    }

    greatest_number.to_string().chars().next().unwrap()
}
