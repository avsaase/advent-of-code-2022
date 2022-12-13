use std::{cmp::Ordering, iter::once};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use serde::Deserialize;

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pair| {
            let (first, second) = pair.split_once("\n").unwrap();
            (
                serde_json::from_str(first).unwrap(),
                serde_json::from_str(second).unwrap(),
            )
        })
        .collect_vec()
}

#[aoc(day13, part1)]
fn part1(packets: &Vec<(Packet, Packet)>) -> u64 {
    packets
        .iter()
        .enumerate()
        .map(|(idx, (first, second))| match compare_vec(&first.0, &second.0) {
            Some(Ordering::Less) | Some(Ordering::Equal) => idx as u64 + 1,
            _ => 0,
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(packets: &Vec<(Packet, Packet)>) -> u64 {
    let mut packets: Vec<Packet> = packets
        .iter()
        .flat_map(|(first, second)| once(first.clone()).chain(once(second.clone())))
        .collect_vec();
    let divider1 = Packet(vec![Data::List(vec![Data::Int(2)])]);
    let divider2 = Packet(vec![Data::List(vec![Data::Int(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(|a, b| compare_vec(&a.0, &b.0).unwrap());

    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, packet)| {
            if *packet == divider1 || *packet == divider2 {
                Some(idx as u64 + 1)
            } else {
                None
            }
        })
        .product()
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
enum Data {
    Int(u8),
    List(Vec<Data>),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
struct Packet(Vec<Data>);

fn compare_data(a: &Data, b: &Data) -> Option<Ordering> {
    match (a, b) {
        (Data::Int(a), Data::Int(b)) => a.partial_cmp(b),
        (Data::List(a), Data::List(b)) => compare_vec(a, b),
        (Data::Int(_), Data::List(b)) => compare_vec(&vec![a.clone()], b),
        (Data::List(a), Data::Int(_)) => compare_vec(a, &vec![b.clone()]),
    }
}

fn compare_vec(a: &Vec<Data>, b: &Vec<Data>) -> Option<Ordering> {
    for (data_a, data_b) in a.iter().zip(b) {
        let comparison = compare_data(data_a, data_b);
        if comparison != Some(Ordering::Equal) {
            return comparison;
        }
    }
    a.len().partial_cmp(&b.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day13.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(&parse_input(PUZZLE_INPUT)), 4821);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 140);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(&parse_input(PUZZLE_INPUT)), 21890);
    }
}
