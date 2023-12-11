use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day06, part1)]
fn generator_day06_part1(input: &str) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut races = vec![];

    let times = lines[0]
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances = lines[1]
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(times.len(), distances.len());

    for (i, time) in times.into_iter().enumerate() {
        races.push(Race {
            time,
            distance: distances[i],
        })
    }

    races
}

#[aoc(day06, part1)]
fn day_06_part_1(input: &[Race]) -> u64 {
    let mut res = vec![];

    for race in input {
        let func = |x: u64| (race.time - x) * x > race.distance;

        let mut counter = 0;

        for i in 0..race.time + 1 {
            if func(i) {
                counter += 1;
            }
        }

        res.push(counter)
    }

    res.into_iter().reduce(|memo, cur| memo * cur).unwrap()
}

#[aoc_generator(day06, part2)]
fn generator_day06_part2(input: &str) -> Race {
    let lines = input.lines().collect::<Vec<_>>();

    let time = lines[0]
        .trim_start_matches("Time:")
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines[1]
        .trim_start_matches("Distance:")
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    Race { time, distance }
}

#[aoc(day06, part2)]
fn day_06_part_2(Race { time, distance }: &Race) -> u64 {
    let func = |x: u64| (*time - x) * x > *distance;

    let mut counter = 0;

    for i in 0..time + 1 {
        if func(i) {
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_generator_day06_part1() {
        let gen = generator_day06_part1(INPUT);

        assert_eq!(
            gen,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        )
    }

    #[test]
    fn test_day06_part1() {
        let gen = generator_day06_part1(INPUT);

        assert_eq!(day_06_part_1(&gen), 288);
    }

    #[test]
    fn test_generator_day06_part2() {
        let gen = generator_day06_part2(INPUT);

        assert_eq!(
            gen,
            Race {
                time: 71530,
                distance: 940200
            }
        )
    }

    #[test]
    fn test_day06_part2() {
        let gen = generator_day06_part2(INPUT);

        assert_eq!(day_06_part_2(&gen), 71503);
    }
}
