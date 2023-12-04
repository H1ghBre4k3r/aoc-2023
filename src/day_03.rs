use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    num: u32,
    neighbours: Vec<char>,
}

fn get_neighbours(lines: &Vec<&str>, col: usize, row: usize) -> Vec<char> {
    let mut neighbours = vec![];

    lines
        .iter()
        .take((row + 2).min(lines.len()))
        .skip(row.max(1) - 1)
        .for_each(|line| {
            if col >= line.len() {
                return;
            }

            let cs = line.chars().collect::<Vec<_>>();
            let c = cs[col];

            if c.is_numeric() || c == '.' {
                return;
            }

            neighbours.push(c)
        });

    neighbours
}

#[aoc_generator(day03, part1)]
fn generator_day03_part1(inp: &str) -> Vec<Number> {
    let mut numbers = vec![];
    let lines = inp.lines().collect::<Vec<_>>();

    let mut num: Option<u32> = None;
    let mut neighbours = vec![];

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let digit = c.to_digit(10).unwrap();
                if num.is_none() && col > 0 {
                    let mut new_neighbours = get_neighbours(&lines, col - 1, row);
                    neighbours.append(&mut new_neighbours);
                }

                num = num.map(|num| num * 10 + digit).or(Some(digit));
                let mut new_neighbours = get_neighbours(&lines, col, row);
                neighbours.append(&mut new_neighbours);
                continue;
            } else if let Some(n) = num {
                let mut new_neighbours = get_neighbours(&lines, col, row);
                neighbours.append(&mut new_neighbours);

                numbers.push(Number {
                    num: n,
                    neighbours: neighbours.clone(),
                });
                neighbours = vec![];
                num = None;
            }
        }

        if let Some(n) = num {
            let mut new_neighbours = get_neighbours(&lines, line.len() - 1, row);
            neighbours.append(&mut new_neighbours);

            numbers.push(Number {
                num: n,
                neighbours: neighbours.clone(),
            });
            neighbours = vec![];
            num = None;
        }
    }

    numbers
}

#[aoc(day03, part1)]
pub fn day03_part1(numbers: &[Number]) -> u32 {
    numbers
        .iter()
        .filter(|Number { neighbours, .. }| !neighbours.is_empty())
        .map(|Number { num, .. }| *num)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_generator_part1() {
        let expected = vec![
            Number {
                num: 467,
                neighbours: vec!['*'],
            },
            Number {
                num: 114,
                neighbours: vec![],
            },
            Number {
                num: 35,
                neighbours: vec!['*'],
            },
            Number {
                num: 633,
                neighbours: vec!['#'],
            },
            Number {
                num: 617,
                neighbours: vec!['*'],
            },
            Number {
                num: 58,
                neighbours: vec![],
            },
            Number {
                num: 592,
                neighbours: vec!['+'],
            },
            Number {
                num: 755,
                neighbours: vec!['*'],
            },
            Number {
                num: 664,
                neighbours: vec!['$'],
            },
            Number {
                num: 598,
                neighbours: vec!['*'],
            },
        ];
        assert_eq!(expected, generator_day03_part1(INPUT));
    }

    #[test]
    fn test_day03_part1() {
        let gen = generator_day03_part1(INPUT);

        assert_eq!(day03_part1(&gen), 4361);
    }
}
