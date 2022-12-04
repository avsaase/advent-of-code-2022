use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let len = l.len();
            let (first, second) = l.split_at(len / 2);
            let common_chars = common_chars(first, second);
            common_chars.iter().map(|c| item_priority(*c)).sum::<u64>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .tuples()
        .into_iter()
        .map(|(a, b, c)| {
            let ab = common_chars(a, b);
            let ac = common_chars(a, c);
            let abc = ab
                .iter()
                .filter(|ab_char| ac.contains(ab_char))
                .unique()
                .collect_vec();
            item_priority(**abc.first().unwrap())
        })
        .sum()
}

fn item_priority(item: char) -> u64 {
    const CHARS: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    CHARS.iter().position(|i| i == &item).unwrap() as u64 + 1
}

fn common_chars(a: &str, b: &str) -> Vec<char> {
    a.chars()
        .filter(|ac| b.chars().contains(ac))
        .unique()
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
    vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw";

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day3.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 70);
    }

    #[test]
    fn priority() {
        assert_eq!(item_priority('p'), 16);
        assert_eq!(item_priority('L'), 38);
        assert_eq!(item_priority('P'), 42);
        assert_eq!(item_priority('v'), 22);
        assert_eq!(item_priority('t'), 20);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(PUZZLE_INPUT), 8105);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(PUZZLE_INPUT), 2363);
    }
}
