use lazy_static::lazy_static;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

lazy_static! {
    static ref CARD_TO_VALUE: HashMap<char, i64> = {
        "AKQJT98765432"
            .chars()
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (card, i as i64))
            .collect()
    };
    static ref HAND_TO_TYPE: HashMap<&'static str, i64> = {
        vec!["1 1 1 1 1", "1 1 1 2", "1 2 2", "1 1 3", "2 3", "1 4", "5"]
            .into_iter()
            .enumerate()
            .map(|(i, count_list)| (count_list, i as i64))
            .collect()
    };
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: i64,
    typ: i64,
}

impl Hand {
    fn new(cards: Vec<char>, bid: i64) -> Hand {
        let typ = Hand::find_type(&cards);
        Hand { cards, bid, typ }
    }

    fn find_type(cards: &Vec<char>) -> i64 {
        let mut m: HashMap<char, usize> = HashMap::new();

        let unique_cards = cards.iter().copied().collect::<HashSet<char>>();
        for card in unique_cards {
            // println!("{card:?}");
            m.insert(card, cards.iter().filter(|&&c| c == card).count());
        }

        let mut sorted_counts = m.values().collect::<Vec<&usize>>(); //.sorted().map(|count| count.to_string()).collect::<Vec<String>>().join(" ");
        sorted_counts.sort();
        let hand_str = sorted_counts
            .iter()
            .map(|&&count| count.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        return HAND_TO_TYPE
            .get(hand_str.as_str())
            .expect("Key error in find_type")
            .to_owned();
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let mut result = self.typ.cmp(&other.typ);
        if result == Ordering::Equal {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                result = CARD_TO_VALUE[&c1].cmp(&CARD_TO_VALUE[&c2]);
                if result != Ordering::Equal {
                    break;
                }
            }
        }

        result
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn p1(input: &str) -> i64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut splt = line.split_whitespace();

            let hand = splt.next().expect("Expected a first element");
            let bid = splt
                .next()
                .expect("Expected a second element")
                .parse::<i64>()
                .expect("Expected a number");

            let cards = hand.chars().collect::<Vec<char>>();

            // println!("{hand:?}");
            // println!("{bid:?}");

            Hand::new(cards, bid)
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    // println!("hands: {hands:?}");

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as i64 + 1))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error while reading input file");

    println!("p1: {}", p1(&input));
}
