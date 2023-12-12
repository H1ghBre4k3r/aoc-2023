use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
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
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hand(Card, Card, Card, Card, Card);

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.2.partial_cmp(&other.2) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.3.partial_cmp(&other.3) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.4.partial_cmp(&other.4)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.trim();
        if s.len() != 5 {
            return Err(());
        }

        let cards = s
            .chars()
            .filter_map(|c| c.to_string().parse::<Card>().ok())
            .collect::<Vec<_>>();

        if cards.len() != 5 {
            return Err(());
        }

        Ok(Hand(cards[0], cards[1], cards[2], cards[3], cards[4]))
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let cards = [self.0, self.1, self.2, self.3, self.4];

        let mut map = HashMap::new();

        for card in &cards {
            let val = map.get(card).unwrap_or(&0);
            map.insert(card, *val + 1);
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
                x => unreachable!("{x:?}"),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

impl FromStr for HandBid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split_whitespace();
        let Some(hand) = segments.next().and_then(|s| s.parse::<Hand>().ok()) else {
            return Err(());
        };

        let Some(bid) = segments.next().and_then(|s| s.parse::<u64>().ok()) else {
            return Err(());
        };

        Ok(HandBid { hand, bid })
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[aoc_generator(day07)]
fn generator_day07(input: &str) -> Vec<HandBid> {
    input
        .lines()
        .map(|line| line.parse::<HandBid>().expect("invalid input"))
        .collect()
}

#[aoc(day07, part1)]
fn day07_part1(handbids: &[HandBid]) -> u64 {
    let mut handbids = handbids.to_vec();
    handbids.sort();
    dbg!(&handbids);
    handbids
        .iter()
        .enumerate()
        .map(|(i, HandBid { bid, .. })| (i as u64 + 1) * *bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_hand_type() {
        assert_eq!(
            Hand(Card::A, Card::A, Card::A, Card::A, Card::A).hand_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand(Card::A, Card::A, Card::A, Card::A, Card::Q).hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand(Card::A, Card::A, Card::A, Card::Q, Card::Q).hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand(Card::A, Card::Q, Card::A, Card::Q, Card::Q).hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand(Card::Two, Card::Q, Card::A, Card::Two, Card::Q).hand_type(),
            HandType::TwoPair
        );
    }

    #[test]
    fn test_hand_type_order() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_generator_day07() {
        use Card::*;
        assert_eq!(
            generator_day07(INPUT),
            vec![
                HandBid {
                    hand: Hand(Three, Two, T, Three, K),
                    bid: 765
                },
                HandBid {
                    hand: Hand(T, Five, Five, J, Five),
                    bid: 684
                },
                HandBid {
                    hand: Hand(K, K, Six, Seven, Seven),
                    bid: 28
                },
                HandBid {
                    hand: Hand(K, T, J, J, T),
                    bid: 220
                },
                HandBid {
                    hand: Hand(Q, Q, Q, J, A),
                    bid: 483
                },
            ]
        )
    }

    #[test]
    fn test_day07_part1() {
        let gen = generator_day07(INPUT);
        assert_eq!(day07_part1(&gen), 6440);
    }
}
