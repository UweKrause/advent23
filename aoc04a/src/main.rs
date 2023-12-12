use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let worth_sum: usize = read_to_string("src/input").unwrap().lines()
        .map(Card::from)
        .map(|c| c.worth())
        .sum();

    println!("{}", worth_sum); // 26346
}


struct Card {
    numbers_winning: Numbers,
    numbers_you_have: Numbers,
}

impl Card {
    fn from(s: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        //      ^  ^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^
        //      |  |                Numbers you have
        //      |  Winning Numbers
        //      Card ID (ignored)
        let mut split = s.split(": ");
        _ = split.next();

        let mut split_numbers = split.next().unwrap().split(" | ");
        let numbers_winning = Numbers::from(split_numbers.next().unwrap());
        let numbers_you_have = Numbers::from(split_numbers.next().unwrap());

        Card { numbers_winning, numbers_you_have }
    }

    fn numbers_matching(&self) -> HashSet<u32> {
        self.numbers_winning.numbers
            .intersection(&self.numbers_you_have.numbers)
            .map(u32::clone)
            .collect()
    }

    fn worth(&self) -> usize {
        match self.numbers_matching().len() {
            0 => 0,
            1 => 1,
            x => 2_usize.pow((x - 1) as u32)
        }
    }
}


struct Numbers {
    numbers: HashSet<u32>,
}

impl Numbers {
    fn from(s: &str) -> Self {
        let numbers = s.split_whitespace()
            .map(str::parse).map(Result::unwrap)
            .collect();
        Self { numbers }
    }
}
