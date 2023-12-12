use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use Card::*;

use crate::Strength::*;

fn main() {
    let mut hands = BinaryHeap::new();
    for line in read_to_string("src/example").unwrap().lines() {
        let hand = Hand::from(line);
        hands.push(hand.clone());
    }

    for (rank, hand) in hands.into_sorted_vec().iter().enumerate() {
        println!("{} {:?}", rank + 1, hand);
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    cards_sorted: Vec<Card>,
    strength: Strength,
    bid: u32,
}

impl PartialEq<Hand> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
            && self.cards_sorted == other.cards_sorted
    }
}

impl Hash for Hand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.hash(state);
        self.cards_sorted.hash(state);
    }
}


impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // "Hands are primarily ordered based on type;
        // for example, every full house is stronger than any three of a kind."
        match self.strength.partial_cmp(&other.strength) {
            Some(ord) => {
                match ord {
                    // "If two hands have the same type,
                    // a second ordering rule takes effect.
                    // Start by comparing the first card in each hand.
                    // If these cards are different,
                    // the hand with the stronger first card is considered stronger.
                    // If the first card in each hand have the same label, however,
                    // then move on to considering the second card in each hand.
                    // If they differ, the hand with the higher second card wins;
                    // otherwise, continue with the third card in each hand,
                    // then the fourth, then the fifth."
                    Ordering::Equal => self.cards.partial_cmp(&other.cards),
                    order_of_not_equal_strength => Some(order_of_not_equal_strength)
                }
            }
            None => { None }
        }
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => { self.cards.cmp(&other.cards) }
            order_of_not_equal_strength => { order_of_not_equal_strength }
        }
    }
}

impl Hand {
    fn from(s: &str) -> Self {
        let cards_str;
        let bid_str;

        if s.contains(" ") {
            let (tmp_cards_str, tmp_bid_str) = s.split_once(" ").unwrap();
            cards_str = tmp_cards_str;
            bid_str = tmp_bid_str;
        } else {
            cards_str = s;
            bid_str = "0";
        }

        assert_eq!(cards_str.len(), 5);

        // Cards stay in order of insertion
        let mut cards: Vec<Card> = Vec::new();
        for c in cards_str.chars() { cards.push(Card::from(c)) }

        // the strength functions and equality criteria expect their Cards sorted by label
        let mut cards_sorted = cards.clone();
        cards_sorted.sort_unstable();
        cards_sorted.reverse();

        let strength = Hand::strength(cards_sorted.clone());

        let bid: u32 = bid_str.parse().unwrap();

        Self { cards, cards_sorted, strength, bid }
    }

    fn strength(cards_sorted: Vec<Card>) -> Strength {
        if Hand::is_five_of_a_kind(&cards_sorted) { return FiveOfAKind; }
        if Hand::is_four_of_a_kind(&cards_sorted) { return FourOfAKind; }
        if Hand::is_full_house(&cards_sorted) { return FullHouse; }
        if Hand::is_three_of_a_kind(&cards_sorted) { return ThreeOfAKind; }
        if Hand::is_two_pair(&cards_sorted) { return TwoPair; }
        if Hand::is_one_pair(&cards_sorted) { return OnePair; }
        return HighCard;
    }

    fn is_five_of_a_kind(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted.
        // If cards are sorted and all cards are the same,
        // the first card is equal to the last card.
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


#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
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


#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Hash, Debug)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,

}


#[cfg(test)]
mod hands {
    use std::collections::BinaryHeap;
    use std::fs::read_to_string;

    use crate::Hand;
    use crate::Strength::*;

    #[test]
    fn parsing() {
        // Hands are considered equal if they contain the same cards,
        // independent of input order
        assert_eq!(Hand::from("KAKA3"), Hand::from("KK3AA"));
        assert_eq!(Hand::from("KKTTT"), Hand::from("TTTKK"));

        // Hands with different cards are not considered equal
        assert_ne!(Hand::from("23456"), Hand::from("789TJ"));

        // Hands with almost the same cards are not considered equal
        assert_ne!(Hand::from("2345T"), Hand::from("2345J"));

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

    #[test]
    fn order() {
        // 33332 and 2AAAA are both four of a kind hands,
        // but 33332 is stronger because its first card is stronger.
        assert!(Hand::from("33332") > Hand::from("2AAAA"));

        // "Similarly, 77888 and 77788 are both a full house,
        // but 77888 is stronger because its third card is stronger
        // (and both hands have the same first and second card).
        assert!(Hand::from("77888") > Hand::from("77788"));
    }

    #[test]
    fn rank() {
        let mut hands = BinaryHeap::new();
        for line in read_to_string("src/example").unwrap().lines() {
            let (hand_str, _) = line.split_once(" ").unwrap();
            hands.push(Hand::from(hand_str));
        }

        // Hands are on the heap, highest rank first
        assert_eq!(hands.pop(), Some(Hand::from("QQQJA")));
        assert_eq!(hands.pop(), Some(Hand::from("T55J5")));
        assert_eq!(hands.pop(), Some(Hand::from("KK677")));
        assert_eq!(hands.pop(), Some(Hand::from("KTJJT")));
        assert_eq!(hands.pop(), Some(Hand::from("32T3K")));
        assert_eq!(hands.pop(), None);
    }
}