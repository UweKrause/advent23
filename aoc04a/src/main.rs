use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    id: u32,
    numbers_winning: Numbers,
    numbers_you_have: Numbers,
}

impl Card {
    fn from(s: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        //      ^  ^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^
        //      |  |                Numbers you have
        //      |  Winning Numbers
        //      Card ID

        let mut split = s.split(": ");
        let id: u32 = split.next().unwrap()
            .split_whitespace().last().unwrap()
            .parse().unwrap();

        let mut split_numbers = split.next().unwrap().split(" | ");
        let numbers_winning = Numbers::from(split_numbers.next().unwrap());
        let numbers_you_have = Numbers::from(split_numbers.next().unwrap());

        Card { id, numbers_winning, numbers_you_have }
    }

    fn intersection(&self) -> HashSet<u32> {
        self.numbers_winning.numbers
            .intersection(&self.numbers_you_have.numbers)
            .map(|i| i.clone())
            .collect()
    }

    fn worth(&self) -> usize {
        match self.intersection().len() {
            0 => 0,
            1 => 1,
            x => 2_usize.pow((x - 1) as u32)
        }
    }
}

#[derive(Debug)]
struct Numbers {
    numbers: HashSet<u32>,
}

impl Numbers {
    fn from(s: &str) -> Self {
        let numbers = s.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self { numbers }
    }
}


fn main() {
    let mut worth_sum = 0;

    for line in read_to_string("src/example").unwrap().lines() {
        let card: Card = Card::from(line);
        println!("{:?} w:{:?} my:{:?} match:{:?} worth:{:?}",
                 card.id,
                 card.numbers_winning.numbers,
                 card.numbers_you_have.numbers,
                 card.intersection(),
                 card.worth()
        );

        worth_sum += card.worth();
    }

    println!("{}", worth_sum); // 13
}
