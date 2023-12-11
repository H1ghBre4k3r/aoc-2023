use std::{num::ParseIntError, str::FromStr, thread};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Ingredients {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Ingredients {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Ingredients::*;
        match s {
            "seed" => Ok(Seed),
            "soil" => Ok(Soil),
            "fertilizer" => Ok(Fertilizer),
            "water" => Ok(Water),
            "light" => Ok(Light),
            "temperature" => Ok(Temperature),
            "humidity" => Ok(Humidity),
            "location" => Ok(Location),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct ConversionRange {
    src_start: usize,
    dest_start: usize,
    length: usize,
}

impl ConversionRange {
    pub fn convert(&self, n: usize) -> Option<usize> {
        if n >= self.src_start && n < self.src_start + self.length {
            return Some(self.dest_start + (n - self.src_start));
        }
        None
    }
}

impl FromStr for ConversionRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(' ');

        let dest_start = numbers.next().expect("invalid input").parse::<usize>()?;
        let src_start = numbers.next().expect("invalid input").parse::<usize>()?;
        let length = numbers.next().expect("invalid input").parse::<usize>()?;

        Ok(ConversionRange {
            src_start,
            dest_start,
            length,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConversionMap {
    from: Ingredients,
    to: Ingredients,
    ranges: Vec<ConversionRange>,
}

impl ConversionMap {
    fn append(&mut self, range: ConversionRange) {
        self.ranges.push(range);
    }

    pub fn convert(&self, num: usize) -> usize {
        for range in &self.ranges {
            if let Some(res) = range.convert(num) {
                return res;
            }
        }
        num
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Input {
    seeds: Vec<usize>,
    maps: Vec<ConversionMap>,
}

impl Input {
    pub fn convert(&self, num: usize) -> usize {
        let mut res = num;
        for map in &self.maps {
            res = map.convert(res);
        }
        res
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let seed_line = lines.next().expect("invalid input");

        let seeds = seed_line
            .trim_start_matches("seeds: ")
            .split_whitespace()
            .map(|num| num.parse::<usize>().expect("invalid input"))
            .collect::<Vec<_>>();

        let mut input = Input {
            seeds,
            maps: vec![],
        };

        let mut map: Option<ConversionMap> = None;

        for line in lines {
            if line.starts_with(|c: char| c.is_alphabetic()) {
                let mut segments = line.split_whitespace();
                let segments = segments
                    .next()
                    .expect("invalid input")
                    .split('-')
                    .collect::<Vec<_>>();

                let from = segments[0].parse::<Ingredients>().expect("invalid input");
                let to = segments[2].parse::<Ingredients>().expect("invalid input");

                map = Some(ConversionMap {
                    from,
                    to,
                    ranges: vec![],
                });
            } else if line.trim().is_empty() {
                if let Some(inner) = map {
                    input.maps.push(inner);
                    map = None;
                };
            } else {
                let range = line.parse::<ConversionRange>().expect("invalid input");
                match map.as_mut() {
                    Some(inner) => inner.append(range),
                    None => unreachable!(),
                }
            }
        }

        if let Some(inner) = map {
            input.maps.push(inner);
        };

        Ok(input)
    }
}

#[aoc_generator(day05)]
fn generator_aoc_day_5(input: &str) -> Input {
    input.parse().expect("invalid input")
}

#[aoc(day05, part1)]
fn day05_part_1(input: &Input) -> usize {
    input
        .seeds
        .iter()
        .map(|seed| input.convert(*seed))
        .reduce(|memo, cur| memo.min(cur))
        .expect("invalid input")
}

#[aoc(day05, part2)]
fn day05_part_2(input: &Input) -> usize {
    let seeds = input.seeds.clone();

    let mut threads = vec![];

    let num = seeds.len() / 2;

    for i in 0..num {
        let start = seeds[i * 2];
        let len = seeds[i * 2 + 1];

        let input = input.clone();

        threads.push(thread::spawn(move || {
            let mut small = usize::MAX;
            for j in start..start + len {
                small = small.min(input.convert(j));
            }
            small
        }));
    }

    let mut small = usize::MAX;

    for ele in threads {
        let res = ele.join().unwrap();
        small = small.min(res);
    }

    small
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    macro_rules! gen_input {
        () => {
            Input {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    ConversionMap {
                        from: Ingredients::Seed,
                        to: Ingredients::Soil,
                        ranges: vec![
                            ConversionRange {
                                src_start: 98,
                                dest_start: 50,
                                length: 2,
                            },
                            ConversionRange {
                                src_start: 50,
                                dest_start: 52,
                                length: 48,
                            },
                        ],
                    },
                    ConversionMap {
                        from: Ingredients::Soil,
                        to: Ingredients::Fertilizer,
                        ranges: vec![
                            ConversionRange {
                                src_start: 15,
                                dest_start: 0,
                                length: 37,
                            },
                            ConversionRange {
                                src_start: 52,
                                dest_start: 37,
                                length: 2,
                            },
                            ConversionRange {
                                src_start: 0,
                                dest_start: 39,
                                length: 15,
                            },
                        ],
                    },
                    ConversionMap {
                        from: Ingredients::Fertilizer,
                        to: Ingredients::Water,
                        ranges: vec![
                            ConversionRange {
                                src_start: 53,
                                dest_start: 49,
                                length: 8,
                            },
                            ConversionRange {
                                src_start: 11,
                                dest_start: 0,
                                length: 42,
                            },
                            ConversionRange {
                                src_start: 0,
                                dest_start: 42,
                                length: 7,
                            },
                            ConversionRange {
                                src_start: 7,
                                dest_start: 57,
                                length: 4,
                            },
                        ],
                    },
                    ConversionMap {
                        from: Ingredients::Water,
                        to: Ingredients::Light,
                        ranges: vec![
                            ConversionRange {
                                src_start: 18,
                                dest_start: 88,
                                length: 7,
                            },
                            ConversionRange {
                                src_start: 25,
                                dest_start: 18,
                                length: 70,
                            },
                        ],
                    },
                    ConversionMap {
                        from: Ingredients::Light,
                        to: Ingredients::Temperature,
                        ranges: vec![
                            ConversionRange {
                                src_start: 77,
                                dest_start: 45,
                                length: 23,
                            },
                            ConversionRange {
                                src_start: 45,
                                dest_start: 81,
                                length: 19,
                            },
                            ConversionRange {
                                src_start: 64,
                                dest_start: 68,
                                length: 13,
                            },
                        ],
                    },
                    ConversionMap {
                        from: Ingredients::Temperature,
                        to: Ingredients::Humidity,
                        ranges: vec![
                            ConversionRange {
                                src_start: 69,
                                dest_start: 0,
                                length: 1,
                            },
                            ConversionRange {
                                src_start: 0,
                                dest_start: 1,
                                length: 69,
                            },
                        ],
                    },
                    ConversionMap {
                        from: Ingredients::Humidity,
                        to: Ingredients::Location,
                        ranges: vec![
                            ConversionRange {
                                src_start: 56,
                                dest_start: 60,
                                length: 37,
                            },
                            ConversionRange {
                                src_start: 93,
                                dest_start: 56,
                                length: 4,
                            },
                        ],
                    },
                ],
            }
        };
    }

    #[test]
    fn test_conversion_range_convert() {
        let range = ConversionRange {
            src_start: 100,
            dest_start: 42,
            length: 69,
        };

        assert_eq!(range.convert(100), Some(42));
        assert_eq!(range.convert(101), Some(43));
        assert_eq!(range.convert(142), Some(84));
        assert_eq!(range.convert(169), None);
        assert_eq!(range.convert(170), None);
        assert_eq!(range.convert(99), None);
    }

    #[test]
    fn test_conversion_range_parse() {
        let input = "17 1337 42";
        assert_eq!(
            ConversionRange::from_str(input),
            Ok(ConversionRange {
                src_start: 1337,
                dest_start: 17,
                length: 42
            })
        )
    }

    #[test]
    fn test_input_parse() {
        let input = Input::from_str(INPUT);
        let expected = gen_input!();

        assert_eq!(input, Ok(expected));
    }

    #[test]
    fn test_input_convert() {
        let input = gen_input!();

        assert_eq!(input.convert(79), 82);
        assert_eq!(input.convert(14), 43);
        assert_eq!(input.convert(55), 86);
        assert_eq!(input.convert(13), 35);
    }

    #[test]
    fn test_part_1() {
        let input = gen_input!();

        assert_eq!(day05_part_1(&input), 35)
    }

    #[test]
    fn test_part_2() {
        let input = gen_input!();

        assert_eq!(day05_part_2(&input), 46)
    }
}
