use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: u32,
    wins: Vec<u32>,
    own: Vec<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split(':');

        let card_info = input.next().expect("invalid input");
        let card_id = card_info
            .trim_start_matches("Card ")
            .trim()
            .parse::<u32>()
            .expect("invalid input");

        let mut game = input.next().expect("invalid input").split(" | ");
        let wins = game
            .next()
            .expect("invalid input")
            .split(' ')
            .filter(|c| !c.trim().is_empty())
            .map(|c| c.trim().parse::<u32>().expect("invalid input"))
            .collect::<Vec<_>>();
        let own = game
            .next()
            .expect("invalid input")
            .split(' ')
            .filter(|c| !c.trim().is_empty())
            .map(|c| c.trim().parse::<u32>().expect("invalid input"))
            .collect::<Vec<_>>();

        Ok(Card {
            id: card_id,
            wins,
            own,
        })
    }
}

#[aoc_generator(day04)]
fn generator_day0(inp: &str) -> Vec<Card> {
    inp.lines()
        .map(|line| line.parse::<Card>().expect("invalid input"))
        .collect()
}

#[aoc(day04, part1)]
fn day04_part_1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .filter_map(|card| {
            let amount = card
                .own
                .iter()
                .filter(|val| card.wins.contains(val))
                .count();

            if amount > 0 {
                Some(2u32.pow(amount as u32 - 1))
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day04, part2)]
fn day04_part_2(cards: &[Card]) -> u32 {
    let mut map = HashMap::new();

    cards.iter().enumerate().for_each(|(index, card)| {
        let amount = card
            .own
            .iter()
            .filter(|val| card.wins.contains(val))
            .count();

        let current_val = map.get(&index).cloned().unwrap_or(1);
        map.insert(index, current_val);

        for i in 0..amount {
            let key = index + i + 1;
            let val = map.get(&key).cloned().unwrap_or(1);
            map.insert(key, val + current_val);
        }
    });
    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 42: 1337 89 123 | 42 17 11";

        let card = Card::from_str(input);

        assert_eq!(
            card,
            Ok(Card {
                id: 42,
                wins: vec![1337, 89, 123],
                own: vec![42, 17, 11]
            })
        )
    }

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day04_part_1() {
        let gen = generator_day0(INPUT);
        assert_eq!(day04_part_1(&gen), 13);
    }

    #[test]
    fn test_day04_part_2() {
        let gen = generator_day0(INPUT);
        assert_eq!(day04_part_2(&gen), 30);
    }
}
