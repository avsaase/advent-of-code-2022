use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, u64> {
    let mut current_dir = Vec::new();
    let mut dir_sizes = HashMap::new();
    for line in input.lines() {
        if let Ok(command) = line.parse::<Command>() {
            match command {
                Command::Cd(direction) => match direction {
                    Direction::Top => current_dir = vec!["root".into()],
                    Direction::Up => {
                        current_dir.pop();
                    }
                    Direction::In(directory) => current_dir.push(directory),
                },
                Command::Ls => (), // Do nothing
            }
        } else if let Ok(file) = line.parse::<File>() {
            for level in 0..current_dir.len() {
                let path = current_dir[0..=level].join("/");
                dir_sizes
                    .entry(path)
                    .and_modify(|x| *x += file.size)
                    .or_insert(file.size);
            }
        }
    }

    dir_sizes
}

#[aoc(day7, part1)]
fn part1(dir_sizes: &HashMap<String, u64>) -> u64 {
    dir_sizes
        .iter()
        .filter_map(|(_, &size)| if size <= 100_000 { Some(size) } else { None })
        .sum()
}

#[aoc(day7, part2)]
fn part2(dir_sizes: &HashMap<String, u64>) -> u64 {
    let total_disk_space = 70000000;
    let required_free_space = 30000000;
    let current_disk_usage = dir_sizes.iter().map(|(_, size)| size).max().unwrap();
    let current_free_space = total_disk_space - current_disk_usage;
    let space_to_clear = required_free_space - current_free_space;
    dir_sizes
        .iter()
        .filter_map(|(_, &size)| if size >= space_to_clear { Some(size) } else { None })
        .min()
        .unwrap()
}

#[derive(Debug, PartialEq, FromStr, Display)]
enum Command {
    #[display("$ cd {0}")]
    Cd(Direction),
    #[display("$ ls")]
    Ls,
}

#[derive(Debug, PartialEq, FromStr, Display)]
enum Direction {
    #[display("/")]
    Top,
    #[display("..")]
    Up,
    #[display("{0}")]
    In(String),
}

#[derive(Debug, PartialEq, FromStr, Display, Clone)]
#[display("{size} {name}")]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, PartialEq, FromStr, Display)]
#[display("dir {name}")]
struct Directory {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day7.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 95437);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(&parse_input(PUZZLE_INPUT)), 1348005);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 24933642);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(&parse_input(PUZZLE_INPUT)), 12785886);
    }
}
