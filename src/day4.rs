use std::{
    cmp::{max, min},
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|pair| {
            let (one, two) = pair.split_once(',').unwrap();
            let one = one.parse().unwrap();
            let two = two.parse().unwrap();
            does_range_include_other(&one, &two)
        })
        .filter(|f| *f)
        .count() as u64
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|pair| {
            let (one, two) = pair.split_once(',').unwrap();
            let one = one.parse().unwrap();
            let two = two.parse().unwrap();
            do_ranges_overlap(&one, &two)
        })
        .filter(|f| *f)
        .count() as u64
}

fn does_range_include_other(a: &Range, b: &Range) -> bool {
    (a.min <= b.min && a.max >= b.max) || (b.min <= a.min && b.max >= a.max)
}

fn do_ranges_overlap(a: &Range, b: &Range) -> bool {
    max(a.min, b.min) <= min(a.max, b.max)
}

struct Range {
    min: u64,
    max: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').unwrap();
        Ok(Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
    2-4,6-8\n\
    2-3,4-5\n\
    5-7,7-9\n\
    2-8,3-7\n\
    6-6,4-6\n\
    2-6,4-8";

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day4.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(PUZZLE_INPUT), 651);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4);
    }
}
