use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Move> {
    input.lines().map(|line| line.parse().unwrap()).collect_vec()
}

#[aoc(day9, part1)]
fn part1(moves: &Vec<Move>) -> u64 {
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut visited_positions = HashSet::from([tail]);

    for move_ in moves {
        for _ in 1..=move_.steps {
            process_step(&mut head, &mut tail, &move_.direction);
            visited_positions.insert(tail);
        }
    }

    visited_positions.len() as u64
}

fn process_step(head: &mut Position, tail: &mut Position, head_step_direction: &Direction) {
    head.step(head_step_direction);
    if tail.is_touching(&head) {
        // Do nothing
    } else if tail.is_in_same_row_col(&head) {
        tail.step(head_step_direction);
    } else {
        let direction = tail.diagonal_direction_to(&head);
        tail.step(&direction);
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug, Display, Copy)]
#[display("({x}, {y})")]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::UpLeft => {
                self.step(&Direction::Up);
                self.step(&Direction::Left);
            }
            Direction::UpRight => {
                self.step(&Direction::Up);
                self.step(&Direction::Right);
            }
            Direction::DownLeft => {
                self.step(&Direction::Down);
                self.step(&Direction::Left);
            }
            Direction::DownRight => {
                self.step(&Direction::Down);
                self.step(&Direction::Right);
            }
        }
    }

    fn diagonal_direction_to(&self, other: &Self) -> Direction {
        let (dist_x, dist_y) = self.distance_to(&other);
        if dist_x > 0 && dist_y > 0 {
            Direction::UpRight
        } else if dist_x > 0 && dist_y < 0 {
            Direction::DownRight
        } else if dist_x < 0 && dist_y > 0 {
            Direction::UpLeft
        } else {
            Direction::DownLeft
        }
    }

    fn is_in_same_row_col(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }

    fn is_touching(&self, other: &Self) -> bool {
        let (x_distance, y_distance) = self.distance_to(other);
        x_distance.abs() <= 1 && y_distance <= 1
    }

    fn distance_to(&self, other: &Self) -> (i32, i32) {
        let x_distance = other.x - self.x;
        let y_distance = other.y - self.y;
        (x_distance, y_distance)
    }
}

#[derive(FromStr, Display, Clone, Debug)]
#[display("{direction} {steps}")]
struct Move {
    direction: Direction,
    steps: u8,
}

#[derive(FromStr, Display, Clone, Debug)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
    #[from_str(ignore)]
    UpLeft,
    #[from_str(ignore)]
    UpRight,
    #[from_str(ignore)]
    DownLeft,
    #[from_str(ignore)]
    DownRight,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day9.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn part1_wrong_solution() {
        assert!(part1(&parse_input(PUZZLE_INPUT)) > 5031);
    }

    #[test]
    fn part1_example_steps() {
        let mut visited_positions = HashSet::new();
        let (mut head, mut tail) = parse_positions(indoc! {"
            ......
            ......
            ......
            ......
            H.....
        "});
        visited_positions.insert(tail);

        //// Move 1: R 4
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ......
                ......
                ......
                TH....
            "},
            &mut visited_positions,
        );

        // Step 2
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ......
                ......
                ......
                sTH...
            "},
            &mut visited_positions,
        );

        // Step 3
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ......
                ......
                ......
                s.TH..
            "},
            &mut visited_positions,
        );

        // Step 4
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ......
                ......
                ......
                s..TH.
            "},
            &mut visited_positions,
        );

        //// Move 2: U 4
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Up,
            indoc! {"
                ......
                ......
                ......
                ....H.
                s..T..
            "},
            &mut visited_positions,
        );

        // Step 2
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Up,
            indoc! {"
                ......
                ......
                ....H.
                ....T.
                s.....
            "},
            &mut visited_positions,
        );

        // Step 3
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Up,
            indoc! {"
                ......
                ....H.
                ....T.
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 4
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Up,
            indoc! {"
                ....H.
                ....T.
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        //// Move 3: L 3
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ...H..
                ....T.
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 2
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ..HT..
                ......
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 3
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                .HT...
                ......
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        //// Move 4: D 1
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Down,
            indoc! {"
                ..T...
                .H....
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        //// Move 4: R 4
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ..T...
                ..H...
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 2
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ..T...
                ...H..
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 3
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ...TH.
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 4
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ....TH
                ......
                ......
                s.....
            "},
            &mut visited_positions,
        );

        //// Move 5: D 1
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Down,
            indoc! {"
                ......
                ....T.
                .....H
                ......
                s.....
            "},
            &mut visited_positions,
        );

        //// Move 5: L 5
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ......
                ....T.
                ....H.
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 2
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ......
                ....T.
                ...H..
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 3
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ......
                ......
                ..HT..
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 4
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ......
                ......
                .HT...
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 5
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Left,
            indoc! {"
                ......
                ......
                HT....
                ......
                s.....
            "},
            &mut visited_positions,
        );

        //// Move 6: R 2
        // Step 1
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ......
                .H....
                ......
                s.....
            "},
            &mut visited_positions,
        );

        // Step 2
        verify_step(
            &mut head,
            &mut tail,
            &Direction::Right,
            indoc! {"
                ......
                ......
                .TH...
                ......
                s.....
            "},
            &mut visited_positions,
        );

        assert_eq!(
            visited_positions,
            parse_visited_positions(indoc! {"
            ..##..
            ...##.
            .####.
            ....#.
            s###..
        "})
        );
    }

    fn verify_step(
        head: &mut Position,
        tail: &mut Position,
        head_step_direction: &Direction,
        map_after_step: &str,
        visited_positions: &mut HashSet<Position>,
    ) {
        process_step(head, tail, head_step_direction);
        assert_eq!((*head, *tail), parse_positions(map_after_step));
        visited_positions.insert(*tail);
    }

    #[test]
    fn parse_position_map() {
        let map = indoc! {"
            ....
            .TH.
            ....
        "};
        assert_eq!(parse_positions(map), (Position { x: 2, y: 1 }, Position { x: 1, y: 1 }));

        let map = indoc! {"
            ....
            .H..
            ..T.
            ....
        "};
        assert_eq!(parse_positions(map), (Position { x: 1, y: 2 }, Position { x: 2, y: 1 }));

        let map = indoc! {"
            ...
            .H.
            ...
        "};
        assert_eq!(parse_positions(map), (Position { x: 1, y: 1 }, Position { x: 1, y: 1 }));
    }

    fn parse_positions(map: &str) -> (Position, Position) {
        let mut head = Position::default();
        let mut tail = Position::default();
        let mut tail_seen = false;
        for (y, line) in map.lines().rev().enumerate() {
            for (x, c) in line.char_indices() {
                if c == 'H' {
                    head.x = x as i32;
                    head.y = y as i32;
                } else if c == 'T' {
                    tail.x = x as i32;
                    tail.y = y as i32;
                    tail_seen = true;
                }
            }
        }
        if !tail_seen {
            tail = head
        }
        (head, tail)
    }

    fn parse_visited_positions(map: &str) -> HashSet<Position> {
        let mut visited_positions = HashSet::new();
        for (y, line) in map.lines().rev().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' || c == 's' {
                    let pos = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    visited_positions.insert(pos);
                }
            }
        }
        visited_positions
    }
}
