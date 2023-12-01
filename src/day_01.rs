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
}
