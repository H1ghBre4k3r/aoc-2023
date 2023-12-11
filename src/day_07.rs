use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Card::*;

        match s {
            "A" => Ok(A),
            "K" => Ok(K),
            "Q" => Ok(Q),
            "J" => Ok(J),
            "T" => Ok(T),
            "9" => Ok(Nine),
            "8" => Ok(Eight),
            "7" => Ok(Seven),
            "6" => Ok(Six),
            "5" => Ok(Five),
            "4" => Ok(Four),
            "3" => Ok(Three),
            "2" => Ok(Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hand(Card, Card, Card, Card, Card);

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        self.hand_type().partial_cmp(&other.hand_type())
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let cards = [self.0, self.1, self.2, self.3, self.4];

        let mut map = HashMap::new();

        for card in &cards {
            let val = map.get(card).unwrap_or(&0);
            map.insert(card, *val);
        }

        match map.keys().len() {
            1 => HandType::FiveOfAKind,
            3 => match map.values().max() {
                Some(3) => HandType::ThreeOfAKind,
                Some(2) => HandType::TwoPair,
                _ => unreachable!(),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            2 => match map.values().max() {
                Some(4) => HandType::FourOfAKind,
                Some(3) => HandType::FullHouse,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
