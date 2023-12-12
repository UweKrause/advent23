use itertools::Itertools;
use Card::*;
use crate::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    strength: Strength,
}

impl Hand {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 5);

        let mut cards: Vec<Card> = Vec::new();
        for c in s.chars() { cards.push(Card::from(c)) }

        cards.sort_unstable();
        cards.reverse();

        let strength = Hand::strength(&cards);

        Self { cards, strength }
    }

    fn strength(v: &Vec<Card>) -> Strength {
        if Hand::is_five_of_a_kind(v) { return FiveOfAKind; }
        if Hand::is_four_of_a_kind(v) { return FourOfAKind; }
        if Hand::is_full_house(v) { return FullHouse; }
        if Hand::is_three_of_a_kind(v) { return ThreeOfAKind; }
        if Hand::is_two_pair(v) { return TwoPair; }
        if Hand::is_one_pair(v) { return OnePair; }
        return HighCard;
    }

    fn is_five_of_a_kind(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted.
        // If cards are sorted and all cards are the same,ä the first card is equal to the last card.
        cs[0] == cs[4]
    }

    fn is_four_of_a_kind(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted
        cs.iter()
            .tuple_windows()
            .any(|(w, x, y, z)| w == x && x == y && y == z)
    }

    fn is_full_house(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted!
        (cs[0] == cs[1] && cs[1] == cs[2]) && (cs[3] == cs[4])
            || (cs[0] == cs[1]) && (cs[2] == cs[3] && cs[3] == cs[4])
    }

    fn is_three_of_a_kind(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted!
        cs.iter()
            .tuple_windows()
            .any(|(x, y, z)| x == y && y == z)
    }

    fn is_two_pair(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted!
        // The two cards forming a pair are always next to each other
        // There are two pairs (AA, BB) and any other card (x)
        // With 5 cards, the x i either
        (cs[1] == cs[2] && cs[3] == cs[4]) // xAABB in front of the pairs
            || (cs[0] == cs[1] && cs[3] == cs[4]) // AAxBB between the pairs
            || (cs[0] == cs[1] && cs[2] == cs[3]) // AABBx or behind the pairs
    }

    fn is_one_pair(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted
        cs.iter()
            .tuple_windows()
            .any(|(x, y)| x == y)
    }
}


#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Card { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

impl Card {
    fn from(s: char) -> Self {
        match s {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!()
        }
    }
}


#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
enum Strength { HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind }


#[cfg(test)]
mod hands {
    use crate::Hand;
    use crate::Strength::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

    #[test]
    fn parsing_and_sorting() {
        // Hands are equal if they contain the same cards, independent of input order
        assert_eq!(Hand::from("KAKA3"), Hand::from("KK3AA"));
        assert_eq!(Hand::from("KKTTT"), Hand::from("TTTKK"));

        // Hands with the same strength but different cards are not considered equal
        assert_ne!(Hand::from("AAATT"), Hand::from("TTTAA"));
    }


    #[test]
    fn strength() {
        assert_eq!(Hand::from("AAAAA").strength, FiveOfAKind);
        assert_eq!(Hand::from("AA8AA").strength, FourOfAKind);
        assert_eq!(Hand::from("23332").strength, FullHouse);
        assert_eq!(Hand::from("TTT98").strength, ThreeOfAKind);
        assert_eq!(Hand::from("23432").strength, TwoPair);
        assert_eq!(Hand::from("A23A4").strength, OnePair);
        assert_eq!(Hand::from("23456").strength, HighCard);
    }
}