use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    // Parse initial stacks
    let mut initial_stacks = Vec::new();
    for line in stacks.lines().rev().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_idx = i / 4;
                if initial_stacks.get(stack_idx).is_none() {
                    initial_stacks.push(Vec::new());
                }
                if let Some(stack) = initial_stacks.get_mut(stack_idx) {
                    stack.push(c);
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
fn part1(input: &(Vec<Stack>, Vec<Move>)) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    for move_ in moves {
        move_crates_one_by_one(&mut stacks, move_);
    }

    get_top_crates(&stacks)
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<Stack>, Vec<Move>)) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();

    for move_ in moves {
        move_crates_all_at_once(&mut stacks, move_);
    }

    get_top_crates(&stacks)
}

type Stack = Vec<Crate>;
type Crate = char;

fn move_crates_one_by_one<'a>(stacks: &'a mut Vec<Stack>, move_: &'a Move) -> &'a mut Vec<Stack> {
    let (amount, from, to) = (
        move_.amount as usize,
        move_.from as usize,
        move_.to as usize,
    );

    for _ in 0..amount {
        let crate_ = stacks.get_mut(from - 1).unwrap().pop().unwrap();
        stacks.get_mut(to - 1).unwrap().push(crate_);
    }
    stacks
}

fn move_crates_all_at_once<'a>(stacks: &'a mut Vec<Stack>, move_: &'a Move) -> &'a mut Vec<Stack> {
    let (amount, from, to) = (
        move_.amount as usize,
        move_.from as usize,
        move_.to as usize,
    );

    let from_len = stacks.get(from - 1).unwrap().len();

    let mut crates_to_move = stacks
        .get_mut(from - 1)
        .unwrap()
        .split_off(from_len - amount);

    stacks.get_mut(to - 1).unwrap().append(&mut crates_to_move);

    stacks
}

fn get_top_crates(stacks: &Vec<Stack>) -> String {
    stacks
        .clone()
        .into_iter()
        .map(|c| c.last().unwrap().to_owned())
        .collect::<String>()
}

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
        let first_stack = vec!['Z', 'N'];
        let second_stack = vec!['M', 'C', 'D'];
        let third_stack = vec!['P'];
        let stacks = vec![first_stack, second_stack, third_stack];
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
