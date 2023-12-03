use aoc_runner_derive::{aoc, aoc_generator};

type CalibrationNumber = u32;

#[aoc_generator(day1, part1)]
fn generator_day01_part1(inp: &str) -> Vec<Vec<CalibrationNumber>> {
    let mut cleared_lines = vec![];

    for line in inp.lines() {
        cleared_lines.push(line.chars().filter_map(|c| c.to_digit(10)).collect());
    }

    cleared_lines
}

const REPLACE: [(&'static str, u32); 18] = [
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

#[aoc_generator(day1, part2)]
fn generator_day01_part2(inp: &str) -> Vec<(CalibrationNumber, CalibrationNumber)> {
    let mut pairs = vec![];

    for line in inp.lines() {
        let mut first = None;
        let mut last = None;
        for i in 0..line.len() {
            for (p, n) in REPLACE {
                if line[i..].starts_with(p) {
                    if first.is_none() {
                        first = Some(n);
                    }
                    last = Some(n)
                }
            }
        }
        pairs.push((
            first.expect("invalid puzzle input"),
            last.expect("invalid puzzle input"),
        ));
    }

    pairs
}

#[aoc(day1, part1)]
pub fn day01_part1(numbers: &[Vec<CalibrationNumber>]) -> u32 {
    numbers
        .iter()
        .map(|line| {
            line.first()
                .map(|first| line.last().map(|last| 10 * *first + *last).unwrap_or(0))
                .unwrap_or(0)
        })
        .reduce(|acc, cur| acc + cur)
        .unwrap_or(0)
}

#[aoc(day1, part2)]
pub fn day01_part_2(numbers: &[(CalibrationNumber, CalibrationNumber)]) -> u32 {
    numbers
        .iter()
        .map(|(first, last)| first * 10 + last)
        .reduce(|acc, cur| acc + cur)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_generator_day_1_part1() {
        assert_eq!(
            generator_day01_part1(INPUT_PART_1),
            vec![vec![1, 2], vec![3, 8], vec![1, 2, 3, 4, 5], vec![7]]
        )
    }

    #[test]
    fn test_part_1() {
        let gen = generator_day01_part1(INPUT_PART_1);

        assert_eq!(day01_part1(&gen), 142);
    }

    const INPUT_PART_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_generator_day_1_part2() {
        assert_eq!(
            generator_day01_part2(INPUT_PART_2),
            vec![(2, 9), (8, 3), (1, 3), (2, 4), (4, 2), (1, 4), (7, 6)]
        )
    }
}
