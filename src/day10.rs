use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::FromStr;

const WIDTH: u8 = 40;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect_vec()
}

#[aoc(day10, part1)]
fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut cycle: u32 = 0;
    let mut x_register: i32 = 1;
    let mut signal_strength: i32 = 0;
    let measure_at = (20..=220).step_by(40).collect_vec();

    for instruction in instructions {
        let cycles = instruction.cycles();
        for _ in 0..cycles {
            cycle += 1;
            if measure_at.contains(&cycle) {
                signal_strength += cycle as i32 * x_register;
            }
        }
        if let Instruction::AddX(x) = instruction {
            x_register += *x as i32;
        }
    }

    signal_strength
}

#[aoc(day10, part2)]
fn part2(instructions: &Vec<Instruction>) -> String {
    let mut cycle: u32 = 0;
    let mut x_register: i32 = 1;

    let mut crt = String::new();
    for instruction in instructions {
        let cycles = instruction.cycles();
        let sprite_positions = (x_register - 1)..=(x_register + 1);
        for _ in 0..cycles {
            let position_to_draw = cycle as i32 % WIDTH as i32;

            if sprite_positions.contains(&position_to_draw) {
                crt.push('#');
            } else {
                crt.push('.');
            }
            if position_to_draw == (WIDTH - 1) as i32 {
                crt.push('\n');
            }

            cycle += 1;
        }
        if let Instruction::AddX(x) = instruction {
            x_register += *x as i32;
        }
    }
    crt
}

#[derive(Debug, FromStr)]
enum Instruction {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    AddX(i8),
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day10.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 13140);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(&parse_input(PUZZLE_INPUT)), 14360);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(EXAMPLE_INPUT)),
            indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
            "}
        );
    }

    #[test]
    fn part2_solution() {
        assert_eq!(
            part2(&parse_input(PUZZLE_INPUT)),
            indoc! {"
            ###...##..#..#..##..####.###..####.####.
            #..#.#..#.#.#..#..#.#....#..#.#.......#.
            ###..#....##...#..#.###..#..#.###....#..
            #..#.#.##.#.#..####.#....###..#.....#...
            #..#.#..#.#.#..#..#.#....#.#..#....#....
            ###...###.#..#.#..#.####.#..#.####.####.
        "}
        );
    }
}
