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

    let mut total_winnings = 0;

    for (rank, hand) in hands.into_sorted_vec().iter().enumerate() {
        let rank_value = (rank + 1) as u32;
        total_winnings += hand.bid * rank_value;
    }
    println!("{}", total_winnings);
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    strength: Strength,
    bid: u32,
}

impl PartialEq<Hand> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
            && self.cards == other.cards
    }
}

impl Hash for Hand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.hash(state);
        self.cards.hash(state);
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

        Self { cards, strength, bid }
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
        cs[0] == cs[1]
            && cs[1] == cs[2]
            && cs[2] == cs[3]
            && cs[3] == cs[4]
            && cs[4] == cs[0]
    }

    fn is_four_of_a_kind(cs: &Vec<Card>) -> bool {
        cs.iter()
            .permutations(4)
            .any(|x| {
                true
                    && /*0          */ x[0] == x[1] && x[0] == x[2] && x[0] == x[3]
                    && x[1] == x[0] /*   1       */ && x[1] == x[2] && x[1] == x[3]
                    && x[2] == x[0] && x[2] == x[1] /*   2       */ && x[2] == x[3]
                    && x[3] == x[0] && x[3] == x[1] && x[3] == x[2] /*   3       */
            })
    }

    fn is_full_house(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted!
        (cs[0] == cs[1] && cs[1] == cs[2]) && (cs[3] == cs[4])
            || (cs[0] == cs[1]) && (cs[2] == cs[3] && cs[3] == cs[4])
    }

    fn is_three_of_a_kind(cs: &Vec<Card>) -> bool {
        cs.iter()
            .permutations(3)
            .any(|x| x[0] == x[1] && x[1] == x[2] && x[2] == x[0])
    }

    fn is_two_pair(cs: &Vec<Card>) -> bool {
        // Assumes that cards are sorted!
        // The two cards forming a pair are always next to each other
        // There are two pairs (AA, BB) and any other card (x)
        // With 5 cards, the x is either:
        (cs[1] == cs[2] && cs[3] == cs[4]) // xAABB in front of the pairs
            || (cs[0] == cs[1] && cs[3] == cs[4]) // AAxBB between the pairs
            || (cs[0] == cs[1] && cs[2] == cs[3]) // AABBx or behind the pairs
    }

    fn is_one_pair(cs: &Vec<Card>) -> bool {
        cs.iter()
            .permutations(2)
            .any(|x| x[0] == x[1])
    }
}


#[derive(Ord, PartialOrd, Eq, Copy, Clone, Hash, Debug)]
enum Card { Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Queen, King, Ace }

impl Card {
    fn from(s: char) -> Self {
        match s {
            'J' => Joker,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,

            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!()
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        /*
        !!! DONT COPY THIS !!!
        This hack works for exactly this problem.
        I have NO IDEA which consequences this has!
        Probably some very strange unforeseeable very ugly to debug consequences...
        !!! DONT COPY THIS !!!
         */
        match self {
            // The joker is considered the same Card as any Card
            Joker => { true }
            s => {
                match other {
                    // If the other Card is a Joker, considered them the same Card
                    Joker => { true }
                    o => { s.cmp(o) == Ordering::Equal }
                }
            }
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
    use std::collections::{BinaryHeap};
    use std::fs::read_to_string;
    use crate::{Hand, Strength};
    use crate::Strength::*;

    #[test]
    fn parsing() {
        // Hands (in the game of "Camel Cards") are considered equal
        // iff they contain the same cards in the same input order
        assert_eq!(Hand::from("KAKA3"), Hand::from("KAKA3"));

        // For the game of "Camel Cards", the order of cards on the hand is important.
        // Usually, for other card games, like e.g. poker,
        // hands with the same cards would have the same value,
        // independent of the cards order.
        // That is NOT the case with Camel Cards!
        assert_ne!(Hand::from("KAKA3"), Hand::from("KK3AA"));
        assert_ne!(Hand::from("KKTTT"), Hand::from("TTTKK"));

        // Hands with different cards are not considered equal
        assert_ne!(Hand::from("23456"), Hand::from("789TJ"));

        // Hands with almost the same cards are not considered equal
        assert_ne!(Hand::from("2345T"), Hand::from("2345Q"));

        // Hands where one card is swapped with a Joker are not considered equal
        assert_ne!(Hand::from("2345T"), Hand::from("2345J"));

        // Hands with the same strength but different cards are not considered equal
        assert_ne!(Hand::from("AAATT"), Hand::from("TTTAA"));
    }


    #[test]
    fn strength() {
        // Normal rules dont change for cases without any Jack/Joker involved
        assert_eq!(Hand::from("AAAAA").strength, FiveOfAKind);
        assert_eq!(Hand::from("AA8AA").strength, FourOfAKind);
        assert_eq!(Hand::from("23332").strength, FullHouse);
        assert_eq!(Hand::from("TTT98").strength, ThreeOfAKind);
        assert_eq!(Hand::from("23432").strength, TwoPair);
        assert_eq!(Hand::from("A23A4").strength, OnePair);
        assert_eq!(Hand::from("23456").strength, HighCard);

        // Things change, when there is a Joker involved!
        assert_eq!(Hand::from("QJJQ2").strength, FourOfAKind);
        assert_eq!(Hand::from("32T3K").strength, OnePair);
        assert_eq!(Hand::from("KK677").strength, TwoPair);
        assert_eq!(Hand::from("T55J5").strength, FourOfAKind);
        assert_eq!(Hand::from("KTJJT").strength, FourOfAKind);
        assert_eq!(Hand::from("QQQJA").strength, FourOfAKind);
    }


    static FIVES: [&str; 8] = [
        "KKKKK", "22222",
        "JKKKK", "KJKKK", "KKJKK", "KKKJK", "KKKKJ",
        "JJJJJ",
    ];

    static FOURS: [&str; 10] = [
        "2KKKK", "K2KKK", "KK2KK", "KKK2K", "KKKK2",
        "2JKKK",
        "2KJKK",
        "2KJJK",
        "2KJKJ",
        "2KJJJ",
    ];

    static FULL_HOUSES: [&str; 5] = [
        "KKKQQ", "QQKKK",
        "QQJKK", "KJKQQ",
        "AAJQQ"
    ];

    static THREES: [&str; 7] = [
        "AAA23", "23334", "AA23A", "KKK23",
        "KKJ23", "AJ5QQ",
        "KJJ23",
    ];

    static TWO_PAIRS: [&str; 3] = [
        "AA3KK", "23344", "44575",
        // No Joker involved, any Joker is either One Pair or at least ThreeOfAKind
        // "J2345" (All distinct, but 1 Joker) -> One Pair
        // "J2245" (One Pair plus one Joker) -> ThreeOfAKind
    ];

    static ONE_PAIRS: [&str; 3] = [
        "AA234",
        "J2345",
        "J9QKA",
    ];

    static HIGH_CARDS: [&str; 2] = [
        "23456",
        "789TQ",
    ];

    fn test_positive(expect: Strength, testees: Vec<&str>) {
        for positive in testees {
            let hand = Hand::from(positive);
            assert_eq!(hand.strength, expect, "{:} {:?}", positive, hand.cards)
        }
    }

    fn test_negative(expect: Strength, testees: Vec<&str>) {
        for negative in testees {
            let hand = Hand::from(negative);
            assert_ne!(hand.strength, expect, "{:} {:?}", negative, hand.cards)
        }
    }


    #[test]
    fn strength_five_of_a_kind() {
        let expect = FiveOfAKind;
        test_positive(expect, FIVES.to_vec());


        test_negative(expect, FOURS.to_vec());
        test_negative(expect, FULL_HOUSES.to_vec());
        test_negative(expect, THREES.to_vec());
        test_negative(expect, TWO_PAIRS.to_vec());
        test_negative(expect, ONE_PAIRS.to_vec());
        test_negative(expect, HIGH_CARDS.to_vec());
    }

    #[test]
    fn strength_four_of_a_kind() {
        let expect = FourOfAKind;
        test_positive(expect, FOURS.to_vec());

        test_negative(expect, FIVES.to_vec());

        test_negative(expect, FULL_HOUSES.to_vec());
        test_negative(expect, THREES.to_vec());
        test_negative(expect, TWO_PAIRS.to_vec());
        test_negative(expect, ONE_PAIRS.to_vec());
        test_negative(expect, HIGH_CARDS.to_vec());
    }


    #[test]
    fn strength_full_house() {
        let expect = FullHouse;
        test_positive(expect, FULL_HOUSES.to_vec());

        test_negative(expect, FIVES.to_vec());
        test_negative(expect, FOURS.to_vec());

        test_negative(expect, THREES.to_vec());
        test_negative(expect, TWO_PAIRS.to_vec());
        test_negative(expect, ONE_PAIRS.to_vec());
        test_negative(expect, HIGH_CARDS.to_vec());
    }

    #[test]
    fn strength_three_of_a_kind() {
        let expect = ThreeOfAKind;
        test_positive(expect, THREES.to_vec());

        test_negative(expect, FIVES.to_vec());
        test_negative(expect, FOURS.to_vec());
        test_negative(expect, FULL_HOUSES.to_vec());

        test_negative(expect, TWO_PAIRS.to_vec());
        test_negative(expect, ONE_PAIRS.to_vec());
        test_negative(expect, HIGH_CARDS.to_vec());
    }

    #[test]
    fn strength_two_pair() {
        let expect = TwoPair;
        test_positive(expect, TWO_PAIRS.to_vec());

        test_negative(expect, FIVES.to_vec());
        test_negative(expect, FOURS.to_vec());
        test_negative(expect, FULL_HOUSES.to_vec());
        test_negative(expect, THREES.to_vec());

        test_negative(expect, ONE_PAIRS.to_vec());
        test_negative(expect, HIGH_CARDS.to_vec());
    }

    #[test]
    fn strength_one_pair() {
        let expect = OnePair;
        test_positive(expect, ONE_PAIRS.to_vec());

        test_negative(expect, FIVES.to_vec());
        test_negative(expect, FOURS.to_vec());
        test_negative(expect, FULL_HOUSES.to_vec());
        test_negative(expect, THREES.to_vec());
        test_negative(expect, TWO_PAIRS.to_vec());

        test_negative(expect, HIGH_CARDS.to_vec());
    }

    #[test]
    fn strength_high_card() {
        let expect = HighCard;
        test_positive(expect, HIGH_CARDS.to_vec());

        test_negative(expect, FIVES.to_vec());
        test_negative(expect, FOURS.to_vec());
        test_negative(expect, FULL_HOUSES.to_vec());
        test_negative(expect, THREES.to_vec());
        test_negative(expect, TWO_PAIRS.to_vec());
        test_negative(expect, ONE_PAIRS.to_vec());
    }

    #[test]
    fn order() {
        // "33332 and 2AAAA are both four of a kind hands,
        // but 33332 is stronger because its first card is stronger."
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

        // Hands are sorted on the heap, highest rank first, lowest rank last
        assert_eq!(hands.pop(), Some(Hand::from("KTJJT")));
        assert_eq!(hands.pop(), Some(Hand::from("QQQJA")));
        assert_eq!(hands.pop(), Some(Hand::from("T55J5")));
        assert_eq!(hands.pop(), Some(Hand::from("KK677")));
        assert_eq!(hands.pop(), Some(Hand::from("32T3K")));
        assert_eq!(hands.pop(), None);
    }
}