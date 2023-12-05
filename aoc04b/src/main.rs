use std::collections::{BTreeMap, HashSet};
use std::fs::read_to_string;

struct Card {
    id: u32,
    worth: u32,
}

impl Card {
    fn from(s: &str) -> Self {

        //       vv
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // ^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // |       split_numbers
        // split_game
        let (split_game, split_numbers) = s.split_once(": ").unwrap();

        //     v
        // Card 1:
        // ^^^^ ^
        // _    split_game_id
        let (_, split_game_id) = split_game.split_once(" ").unwrap();

        //               vvv
        // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // ^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^
        // |                split_numbers_you_have
        // split_numbers_winning
        let (split_numbers_winning, split_numbers_you_have) = split_numbers.split_once(" | ").unwrap();

        // "1" -> 1
        let id: u32 = split_game_id.trim().parse().unwrap();

        let numbers_winning = Numbers::from(split_numbers_winning);
        let numbers_you_have = Numbers::from(split_numbers_you_have);

        let worth = Self::worth(numbers_winning, numbers_you_have);

        Self { id, worth }
    }

    fn worth(numbers_winning: Numbers, numbers_you_have: Numbers) -> u32 {
        numbers_winning.numbers
            .intersection(&numbers_you_have.numbers)
            .count() as u32
    }
}


struct Numbers {
    numbers: HashSet<u32>,
}

impl Numbers {
    fn from(s: &str) -> Self {
        //   v  vv v  v  vv v  v
        // 83 86  6 31 17  9 48 53
        // ^^ ^^  ^ ^^ ^^  ^ ^^ ^^
        // numbers
        let numbers = s.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self { numbers }
    }
}


fn main() {

    // ids in input start at 1.
    // in an attempt to minimize the confusion,
    // lets also do this

    let mut pile: BTreeMap<u32, u32> = BTreeMap::new();

    // For every card
    // Add the original card to the pile of cards
    // pile[current.id] += 1
    //
    // Add one for as much tickets as the current ticket is worth:
    // for current.id+1..current.id+1+current.worth
    //   pile[current.id+1] += current.worth
    for card in read_to_string("src/input").unwrap().lines().into_iter() {
        let card = Card::from(card);

        let id = card.id;

        // ToDo: I got neither .entry() nor DefaultBtreeMap working...
        let current = pile.get(&id).unwrap_or(&0);
        pile.insert(id, current + 1);

        let current = pile.get(&id).unwrap_or(&0).to_owned();
        let from = card.id + 1;
        let to = from + card.worth;
        for next_nth in from..to {
            pile.insert(next_nth, pile.get(&next_nth).unwrap_or(&0) + current);
        }
    }

    let card_sum: u32 = pile.values().sum();
    println!("{}", card_sum)
}
