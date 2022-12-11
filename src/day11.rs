use std::{collections::VecDeque, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| block.trim().parse().unwrap())
        .collect_vec()
}

#[aoc(day11, part1)]
fn part1(monkeys: &Vec<Monkey>) -> u64 {
    let mut monkeys = monkeys.to_owned();
    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let monkey = monkeys[monkey_idx].clone();
            let n_items = monkey.items.0.len();
            for item in &monkey.items.0 {
                let worry_level = monkey.operation.apply(*item) / 3;
                let throw_to = monkey.test(worry_level);
                monkeys[throw_to as usize].items.0.push_back(worry_level);
            }
            monkeys[monkey_idx].items.0.clear();
            monkeys[monkey_idx].items_inspected += n_items as u64;
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part2)]
fn part2(monkeys: &Vec<Monkey>) -> u64 {
    let mut monkeys = monkeys.to_owned();
    let common_divisor: u64 = monkeys.iter().map(|m| m.test.divisor as u64).product();
    for _round in 0..10_000 {
        for monkey_idx in 0..monkeys.len() {
            let monkey = monkeys[monkey_idx].clone();
            let n_items = monkey.items.0.len();
            for item in &monkey.items.0 {
                let worry_level = monkey.operation.apply(*item) % common_divisor;
                let throw_to = monkey.test(worry_level);
                monkeys[throw_to as usize].items.0.push_back(worry_level);
            }
            monkeys[monkey_idx].items.0.clear();
            monkeys[monkey_idx].items_inspected += n_items as u64;
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[derive(Debug, FromStr, Clone)]
#[display(
    "Monkey {_id}:
  Starting items: {items}
  Operation: {operation}
  Test: divisible by {test.divisor}
    If true: throw to monkey {test.if_divisible}
    If false: throw to monkey {test.if_not_divisible}"
)]
struct Monkey {
    _id: u8,
    items: Items,
    operation: Operation,
    #[from_str(default)]
    test: Test,
    #[from_str(default)]
    items_inspected: u64,
}

impl Monkey {
    fn test(&self, worry_level: u64) -> u8 {
        self.test.apply(worry_level)
    }
}

#[derive(Debug, Clone)]
struct Items(VecDeque<u64>);

impl FromStr for Items {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Items(s.split(", ").map(|i| i.parse().unwrap()).collect()))
    }
}

#[derive(Debug, Display, FromStr, Clone)]
enum Operation {
    #[display("new = old * old")]
    Square,
    #[display("new = old * {0}")]
    Multiply(u8),
    #[display("new = old + {0}")]
    Add(u8),
}

impl Operation {
    fn apply(&self, val: u64) -> u64 {
        match self {
            Operation::Square => val * val,
            Operation::Multiply(x) => val * *x as u64,
            Operation::Add(x) => val + *x as u64,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Test {
    divisor: u64,
    if_divisible: u8,
    if_not_divisible: u8,
}

impl Test {
    fn apply(&self, val: u64) -> u8 {
        if val % self.divisor == 0 {
            self.if_divisible
        } else {
            self.if_not_divisible
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    Monkey 0:
      Starting items: 79, 98
      Operation: new = old * 19
      Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3
  
    Monkey 1:
      Starting items: 54, 65, 75, 74
      Operation: new = old + 6
      Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0
  
    Monkey 2:
      Starting items: 79, 60, 97
      Operation: new = old * old
      Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3
    
    Monkey 3:
      Starting items: 74
      Operation: new = old + 3
      Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day11.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 10605);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(&parse_input(PUZZLE_INPUT)), 120384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 2713310158);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(&parse_input(PUZZLE_INPUT)), 32059801242);
    }
}
