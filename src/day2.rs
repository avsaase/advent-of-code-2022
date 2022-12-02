use std::str::FromStr;

use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let line = l.split_once(" ").unwrap();
            let their_action = line.0.parse().unwrap();
            let my_action = line.1.parse().unwrap();
            points_for_match(their_action, my_action)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let line = l.split_once(" ").unwrap();
            let their_action = line.0.parse().unwrap();
            let desired_outcome = line.1.parse().unwrap();
            let my_action = action_for_outcome(their_action, desired_outcome);
            points_for_match(their_action, &my_action)
        })
        .sum()
}

enum Action {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for &Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(&Action::Rock),
            "B" | "Y" => Ok(&Action::Paper),
            "C" | "Z" => Ok(&Action::Scissors),
            _ => unreachable!(),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for &Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(&Outcome::Loss),
            "Y" => Ok(&Outcome::Draw),
            "Z" => Ok(&Outcome::Win),
            _ => unreachable!(),
        }
    }
}

fn calculate_outcome(their_action: &Action, my_action: &Action) -> Outcome {
    match (their_action, my_action) {
        (Action::Rock, Action::Rock) => Outcome::Draw,
        (Action::Rock, Action::Paper) => Outcome::Win,
        (Action::Rock, Action::Scissors) => Outcome::Loss,
        (Action::Paper, Action::Rock) => Outcome::Loss,
        (Action::Paper, Action::Paper) => Outcome::Draw,
        (Action::Paper, Action::Scissors) => Outcome::Win,
        (Action::Scissors, Action::Rock) => Outcome::Win,
        (Action::Scissors, Action::Paper) => Outcome::Loss,
        (Action::Scissors, Action::Scissors) => Outcome::Draw,
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
    match (their_action, outcome) {
        (Action::Rock, Outcome::Win) => Action::Paper,
        (Action::Rock, Outcome::Draw) => Action::Rock,
        (Action::Rock, Outcome::Loss) => Action::Scissors,
        (Action::Paper, Outcome::Win) => Action::Scissors,
        (Action::Paper, Outcome::Draw) => Action::Paper,
        (Action::Paper, Outcome::Loss) => Action::Rock,
        (Action::Scissors, Outcome::Win) => Action::Rock,
        (Action::Scissors, Outcome::Draw) => Action::Scissors,
        (Action::Scissors, Outcome::Loss) => Action::Paper,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
    A Y\n\
    B X\n\
    C Z\n";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 12);
    }
}
