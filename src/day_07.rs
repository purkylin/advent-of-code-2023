use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn task1() -> usize {
    let mut input = read(false);
    input.sort_by(|a, b| a.cmp(b));
    let value: usize = input
        .iter()
        .enumerate()
        // .inspect(|(i, v)| println!("hi {} {} {} => {:?}", i + 1, v.0, v.1, v.0.card_type()))
        .map(|(i, v)| (i + 1) * v.1)
        .sum();
    value
}

pub fn task2() -> usize {
    0
}

fn read(sample: bool) -> Vec<(Hand, usize)> {
    let data: &str;
    if sample {
        data = include_str!("data/sample.txt");
    } else {
        data = include_str!("data/day-07.txt");
    }

    let lines = data
        .split("\n")
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();
            (left.into(), right.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    lines
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (v1, v2) = (self.card_type(), other.card_type());
        if v1 == v2 {
            self.cards.partial_cmp(&other.cards)
        } else {
            v1.partial_cmp(&v2)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (v1, v2) = (self.card_type(), other.card_type());
        if v1 == v2 {
            self.cards.cmp(&other.cards)
        } else {
            v1.cmp(&v2)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.card_type() == other.card_type() {
            return self.cards == other.cards;
        }

        return false;
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.cards.iter().map(|x| x.0).collect();
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Clone, Copy, Hash)]
struct Card(char);

impl Card {
    fn raw(&self) -> u8 {
        match self.0 {
            '2'..='9' => self.0.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 1, //11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.raw().cmp(&other.raw())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl Hand {
    fn card_type_old(&self) -> CardType {
        let dict = self.cards.iter().fold(HashMap::new(), |mut acc, v| {
            acc.entry(v).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        let mut values = dict.values().copied().collect::<Vec<_>>();
        values.sort();

        match values[..] {
            [5] => CardType::Five,
            [1, 4] => CardType::Four,
            [2, 3] => CardType::Full,
            [1, 1, 3] => CardType::Three,
            [1, 2, 2] => CardType::TwoPair,
            [1, 1, 1, 2] => CardType::OnePair,
            [1, 1, 1, 1, 1] => CardType::High,
            _ => panic!("Unknown card type, {:?}", values),
        }
    }

    fn card_type(&self) -> CardType {
        let dict = self.cards.iter().fold(HashMap::new(), |mut acc, v| {
            acc.entry(v).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        let mut back_dict = dict.clone();
        let j_card = Card('J');
        let j_count = back_dict.get(&j_card).unwrap_or(&0).to_owned();
        back_dict.remove(&j_card);
        let mut values = back_dict.values().copied().collect::<Vec<_>>();
        values.sort();

        match j_count {
            5 => CardType::Five,
            4 => CardType::Five,
            3 if values.len() == 2 => CardType::Four,
            3 if values.len() == 1 => CardType::Five,
            2 if values.len() == 3 => CardType::Three,
            2 if values.len() == 2 => CardType::Four,
            2 if values.len() == 1 => CardType::Five,
            1 if values.len() == 4 => CardType::OnePair,
            1 if values.len() == 3 => CardType::Three,
            1 if values == [2, 2] => CardType::Full,
            1 if values == [1, 3] => CardType::Four,
            1 if values.len() == 1 => CardType::Five,
            0 => self.card_type_old(),
            _ => panic!("Invalid combine: {:?}", &values),
        }
    }
}

impl Hand {
    fn new(s: &str) -> Self {
        Self {
            cards: s.chars().map(|c| Card(c)).collect(),
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

// #[test]
// fn test_cards() {
//     assert!(CardType::Five > CardType::Four);
//     assert!(CardType::High < CardType::Full);

//     let hand1: Hand = "A2A4T".into();
//     assert_eq!(hand1.card_type(), CardType::OnePair);

//     let hand2: Hand = "A2A2T".into();
//     assert_eq!(hand2.card_type(), CardType::TwoPair);

//     let hand: Hand = "A2A22".into();
//     assert_eq!(hand.card_type(), CardType::Full);

//     let hand: Hand = "A4567".into();
//     assert_eq!(hand.card_type(), CardType::High);

//     let hand: Hand = "TTTTT".into();
//     assert_eq!(hand.card_type(), CardType::Five);

//     let hand: Hand = "TTTT4".into();
//     assert_eq!(hand.card_type(), CardType::Four);

//     let hand: Hand = "2TTT4".into();
//     assert_eq!(hand.card_type(), CardType::Three);

//     println!("cc {:?} {:?}", hand1.card_type(), hand2.card_type());
//     assert!(hand1 < hand2);
// }

// #[test]
// fn test_sample() {
//     let mut input = read(true);
//     input.sort_by(|a, b| a.cmp(b));
//     let value: usize = input
//         .iter()
//         .enumerate()
//         .inspect(|(i, v)| println!("hi {} {} {} => {:?}", i + 1, v.0, v.1, v.0.card_type()))
//         .map(|(i, v)| (i + 1) * v.1)
//         .sum();
//     assert_eq!(value, 6440)
// }

#[test]
fn test_sample() {
    let n: Hand = "KK677".into();
    assert_eq!(n.card_type(), CardType::TwoPair);

    let mut input = read(true);
    input.sort_by(|a, b| a.cmp(b));
    let value: usize = input
        .iter()
        .enumerate()
        .inspect(|(i, v)| println!("hi {} {} {} => {:?}", i + 1, v.0, v.1, v.0.card_type()))
        .map(|(i, v)| (i + 1) * v.1)
        .sum();
    assert_eq!(value, 5905)
}
