use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    calories_per_elf(input).into_iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    calories_per_elf(input)
        .into_iter()
        .sorted()
        .rev()
        .take(3)
        .sum::<u64>()
}

fn calories_per_elf(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INTPUT: &str = "\
    1000\n\
    2000\n\
    3000\n\
    4000\n\
    \n\
    5000\n\
    6000\n\
    \n\
    7000\n\
    8000\n\
    9000\n\
    \n\
    10000\n";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INTPUT), 24_000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INTPUT), 45_000);
    }
}
