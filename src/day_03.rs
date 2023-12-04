use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    num: u32,
    neighbours: Vec<(char, usize, usize)>,
}

fn get_neighbours(lines: &Vec<&str>, col: usize, row: usize) -> Vec<(char, usize, usize)> {
    let mut neighbours = vec![];

    let lower_bound = row.max(1) - 1;

    lines
        .iter()
        .take((row + 2).min(lines.len()))
        .skip(lower_bound)
        .enumerate()
        .for_each(|(y, line)| {
            if col >= line.len() {
                return;
            }

            let cs = line.chars().collect::<Vec<_>>();
            let c = cs[col];

            if c.is_numeric() || c == '.' {
                return;
            }

            neighbours.push((c, col, lower_bound + y))
        });

    neighbours
}

#[aoc_generator(day03)]
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

#[aoc(day03, part2)]
pub fn day03_part2(numbers: &[Number]) -> u32 {
    let set = numbers
        .iter()
        .flat_map(|Number { neighbours, .. }| neighbours)
        .cloned()
        .collect::<HashSet<_>>();

    set.into_iter()
        .map(|symbol| {
            let nums = numbers
                .iter()
                .filter(|Number { neighbours, .. }| neighbours.contains(&symbol))
                .map(|Number { num, .. }| *num)
                .collect::<Vec<_>>();

            nums
        })
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.into_iter().reduce(|memo, cur| memo * cur).unwrap_or(0))
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
                neighbours: vec![('*', 3, 1)],
            },
            Number {
                num: 114,
                neighbours: vec![],
            },
            Number {
                num: 35,
                neighbours: vec![('*', 3, 1)],
            },
            Number {
                num: 633,
                neighbours: vec![('#', 6, 3)],
            },
            Number {
                num: 617,
                neighbours: vec![('*', 3, 4)],
            },
            Number {
                num: 58,
                neighbours: vec![],
            },
            Number {
                num: 592,
                neighbours: vec![('+', 5, 5)],
            },
            Number {
                num: 755,
                neighbours: vec![('*', 5, 8)],
            },
            Number {
                num: 664,
                neighbours: vec![('$', 3, 8)],
            },
            Number {
                num: 598,
                neighbours: vec![('*', 5, 8)],
            },
        ];
        assert_eq!(expected, generator_day03_part1(INPUT));
    }

    #[test]
    fn test_day03_part1() {
        let gen = generator_day03_part1(INPUT);

        assert_eq!(day03_part1(&gen), 4361);
    }

    #[test]
    fn test_day03_part2() {
        let gen = generator_day03_part1(INPUT);

        assert_eq!(day03_part2(&gen), 467835);
    }
}
