use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Stacks, Vec<Move>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    // Parse initial stacks
    let mut initial_stacks = Stacks(Vec::new());
    for line in stacks.lines().rev().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_idx = i / 4;
                if initial_stacks.0.get(stack_idx).is_none() {
                    initial_stacks.0.push(Stack(Vec::new()));
                }
                if let Some(stack) = initial_stacks.0.get_mut(stack_idx) {
                    stack.0.push(Crate(c));
                }
            }
        }
    }

    // Parse moves
    let moves = moves
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect::<Vec<_>>();

    (initial_stacks, moves)
}

#[aoc(day5, part1)]
fn part1(input: &(Stacks, Vec<Move>)) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    for move_ in moves {
        stacks.move_crates_one_by_one(move_);
    }

    stacks.get_top_crates()
}

#[aoc(day5, part2)]
fn part2(input: &(Stacks, Vec<Move>)) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    for move_ in moves {
        stacks.move_crates_all_at_once(move_);
    }

    stacks.get_top_crates()
}

#[derive(Debug, PartialEq, Clone)]
struct Stack(Vec<Crate>);

#[derive(Debug, PartialEq, Clone)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn move_crates_one_by_one(&mut self, move_: &Move) -> &mut Self {
        let (amount, from, to) = (
            move_.amount as usize,
            move_.from as usize,
            move_.to as usize,
        );

        for _ in 0..amount {
            let crate_ = self.0.get_mut(from - 1).unwrap().0.pop().unwrap();
            self.0.get_mut(to - 1).unwrap().0.push(crate_);
        }
        self
    }

    fn move_crates_all_at_once(&mut self, move_: &Move) -> &mut Self {
        let (amount, from, to) = (
            move_.amount as usize,
            move_.from as usize,
            move_.to as usize,
        );

        let from_len = self.0.get(from - 1).unwrap().0.len();

        let mut crates_to_move = self
            .0
            .get_mut(from - 1)
            .unwrap()
            .0
            .drain((from_len - amount)..)
            .collect_vec();

        self.0
            .get_mut(to - 1)
            .unwrap()
            .0
            .append(&mut crates_to_move);

        self
    }

    fn get_top_crates(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .map(|c| c.0.last().unwrap().0)
            .collect::<String>()
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Crate(char);

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {amount} from {from} to {to}")]
struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day5.txt");

    #[test]
    fn parse_initial_stacks_example() {
        let first_stack = Stack(vec![Crate('Z'), Crate('N')]);
        let second_stack = Stack(vec![Crate('M'), Crate('C'), Crate('D')]);
        let third_stack = Stack(vec![Crate('P')]);
        let stacks = Stacks(vec![first_stack, second_stack, third_stack]);
        let (initial_stacks, _) = parse_input(EXAMPLE_INPUT);
        assert_eq!(initial_stacks, stacks);
    }

    #[test]
    fn parse_moves_example() {
        let moves = vec![
            Move {
                amount: 1,
                from: 2,
                to: 1,
            },
            Move {
                amount: 3,
                from: 1,
                to: 3,
            },
            Move {
                amount: 2,
                from: 2,
                to: 1,
            },
            Move {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];
        let (_, parsed_moves) = parse_input(EXAMPLE_INPUT);
        assert_eq!(parsed_moves, moves);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), "CMZ".to_string());
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(&parse_input(PUZZLE_INPUT)), "ZWHVFWQWW".to_string());
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), "MCD".to_string());
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(&parse_input(PUZZLE_INPUT)), "HZFZCCWWV".to_string());
    }
}
