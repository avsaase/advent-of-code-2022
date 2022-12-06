use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    for (idx, (a, b, c, d)) in input.chars().tuple_windows::<(_, _, _, _)>().enumerate() {
        if [a, b, c, d].iter().unique().count() == 4 {
            return (idx + 4) as u64;
        }
    }
    panic!("Should have found solution");
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    for (idx, slice) in input.as_bytes().windows(14).enumerate() {
        if slice.iter().unique().count() == 14 {
            return (idx + 14) as u64;
        }
    }
    panic!("Should have found solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1_INPUTS: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    const EXAMPLE_PART2_INPUTS: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day6.txt");

    #[test]
    fn part1_example() {
        EXAMPLE_PART1_INPUTS
            .iter()
            .for_each(|(input, output)| assert_eq!(part1(*input), *output as u64))
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(PUZZLE_INPUT), 1578);
    }

    #[test]
    fn part2_example() {
        EXAMPLE_PART2_INPUTS
            .iter()
            .for_each(|(input, output)| assert_eq!(part2(*input), *output as u64))
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(PUZZLE_INPUT), 2178);
    }
}
