use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Information {
    color: Color,
    amount: u32,
}

impl FromStr for Information {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        let amount = parts[0].parse::<u32>().expect("invalid input");
        let color = parts[1].parse::<Color>().expect("invalid input");
        Ok(Information { color, amount })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    information: Vec<Information>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split(':').collect::<Vec<_>>();

        let id = segments.remove(0).split(' ').collect::<Vec<_>>()[1]
            .parse::<u32>()
            .expect("invalid input");

        let mut information = vec![];

        segments[0].split("; ").for_each(|elem| {
            elem.split(", ")
                .for_each(|game| information.push(game.parse().expect("invalid input")))
        });

        Ok(Game { id, information })
    }
}

#[aoc_generator(day2)]
fn generator_day02_part1(inp: &str) -> Vec<Game> {
    inp.lines()
        .map(|line| line.parse::<Game>().expect("invalid input"))
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn day02_part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|Game { information, .. }| {
            information.iter().all(|Information { color, amount }| {
                *amount
                    <= match color {
                        Color::Red => RED,
                        Color::Green => GREEN,
                        Color::Blue => BLUE,
                    }
            })
        })
        .map(|Game { id, .. }| *id)
        .sum()
}

#[aoc(day2, part2)]
fn day02_part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|Game { information, .. }| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for Information { color, amount } in information {
                match color {
                    Color::Red => red = red.max(*amount),
                    Color::Green => green = green.max(*amount),
                    Color::Blue => blue = blue.max(*amount),
                }
            }

            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_generator_part_1() {
        let result = vec![
            Game {
                id: 1,
                information: vec![
                    Information {
                        amount: 3,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 4,
                        color: Color::Red,
                    },
                    Information {
                        amount: 1,
                        color: Color::Red,
                    },
                    Information {
                        amount: 2,
                        color: Color::Green,
                    },
                    Information {
                        amount: 6,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 2,
                        color: Color::Green,
                    },
                ],
            },
            Game {
                id: 2,
                information: vec![
                    Information {
                        amount: 1,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 2,
                        color: Color::Green,
                    },
                    Information {
                        amount: 3,
                        color: Color::Green,
                    },
                    Information {
                        amount: 4,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 1,
                        color: Color::Red,
                    },
                    Information {
                        amount: 1,
                        color: Color::Green,
                    },
                    Information {
                        amount: 1,
                        color: Color::Blue,
                    },
                ],
            },
            Game {
                id: 3,
                information: vec![
                    Information {
                        amount: 8,
                        color: Color::Green,
                    },
                    Information {
                        amount: 6,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 20,
                        color: Color::Red,
                    },
                    Information {
                        amount: 5,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 4,
                        color: Color::Red,
                    },
                    Information {
                        amount: 13,
                        color: Color::Green,
                    },
                    Information {
                        amount: 5,
                        color: Color::Green,
                    },
                    Information {
                        amount: 1,
                        color: Color::Red,
                    },
                ],
            },
            Game {
                id: 4,
                information: vec![
                    Information {
                        amount: 1,
                        color: Color::Green,
                    },
                    Information {
                        amount: 3,
                        color: Color::Red,
                    },
                    Information {
                        amount: 6,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 3,
                        color: Color::Green,
                    },
                    Information {
                        amount: 6,
                        color: Color::Red,
                    },
                    Information {
                        amount: 3,
                        color: Color::Green,
                    },
                    Information {
                        amount: 15,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 14,
                        color: Color::Red,
                    },
                ],
            },
            Game {
                id: 5,
                information: vec![
                    Information {
                        amount: 6,
                        color: Color::Red,
                    },
                    Information {
                        amount: 1,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 3,
                        color: Color::Green,
                    },
                    Information {
                        amount: 2,
                        color: Color::Blue,
                    },
                    Information {
                        amount: 1,
                        color: Color::Red,
                    },
                    Information {
                        amount: 2,
                        color: Color::Green,
                    },
                ],
            },
        ];

        assert_eq!(generator_day02_part1(INPUT), result);
    }

    #[test]
    fn test_day02_part_1() {
        let gen = generator_day02_part1(INPUT);

        assert_eq!(day02_part1(&gen), 8);
    }

    #[test]
    fn test_day02_part_2() {
        let gen = generator_day02_part1(INPUT);

        assert_eq!(day02_part2(&gen), 2286);
    }
}
