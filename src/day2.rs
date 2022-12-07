use std::str::FromStr;

use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let line = l.split_once(' ').unwrap();
            let their_action = line.0.parse().unwrap();
            let my_action = line.1.parse().unwrap();
            points_for_match(&their_action, &my_action)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let line = l.split_once(' ').unwrap();
            let their_action = line.0.parse::<Action>().unwrap();
            let desired_outcome = line.1.parse().unwrap();
            let my_action = action_for_outcome(&their_action, &desired_outcome);
            points_for_match(&their_action, &my_action)
        })
        .sum()
}

#[derive(Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Action {
    fn from(x: u8) -> Self {
        match x {
            0 => Action::Rock,
            1 => Action::Paper,
            2 => Action::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
enum Outcome {
    Draw,
    Win,
    Loss,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => unreachable!(),
        }
    }
}

fn calculate_outcome(their_action: &Action, my_action: &Action) -> Outcome {
    let their_action_idx = their_action.clone() as u8;
    let my_action_idx = my_action.clone() as u8;
    if my_action_idx == (their_action_idx + 1) % 3 {
        Outcome::Win
    } else if my_action_idx == their_action_idx {
        Outcome::Draw
    } else {
        Outcome::Loss
    }
}

fn points_for_action(action: &Action) -> u64 {
    match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    }
}

fn points_for_outcome(outcome: &Outcome) -> u64 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

fn points_for_match(their_action: &Action, my_action: &Action) -> u64 {
    let poits_for_action = points_for_action(my_action);
    let poits_for_result = points_for_outcome(&calculate_outcome(their_action, my_action));
    poits_for_action + poits_for_result
}

fn action_for_outcome(their_action: &Action, outcome: &Outcome) -> Action {
    let their_action_idx = their_action.clone() as u8;
    let outcome_idx = outcome.clone() as u8;
    let my_action = (their_action_idx + outcome_idx) % 3;
    my_action.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
    A Y\n\
    B X\n\
    C Z";

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 12);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(PUZZLE_INPUT), 13565);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(PUZZLE_INPUT), 12424);
    }
}
