use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{s, Array2};

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Array2<u8> {
    let rows = input.lines().count();
    let cols = input.lines().next().map(|row| row.chars().count()).unwrap();

    let mut array = Array2::<u8>::zeros((rows, cols));

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, val) in row.chars().enumerate() {
            let val = val.to_digit(10).unwrap();
            array[[row_idx, col_idx]] = val as u8;
        }
    }

    array
}

#[aoc(day8, part1)]
fn part1(tree_heigths: &Array2<u8>) -> u64 {
    let rows = tree_heigths.nrows();
    let cols = tree_heigths.ncols();

    let mut n_visible = (2 * rows + 2 * (cols - 2)) as u64;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let val = tree_heigths[[i, j]];
            let is_visible_from_left = tree_heigths.slice(s![i, 0..j]).iter().max().unwrap().clone() < val;
            let is_visible_from_right = tree_heigths.slice(s![i, (j + 1)..]).iter().max().unwrap().clone() < val;
            let is_visible_from_top = tree_heigths.slice(s![0..i, j]).iter().max().unwrap().clone() < val;
            let is_visible_from_bottom = tree_heigths.slice(s![(i + 1).., j]).iter().max().unwrap().clone() < val;

            if is_visible_from_left || is_visible_from_right || is_visible_from_top || is_visible_from_bottom {
                n_visible += 1;
            }
        }
    }
    n_visible
}

#[aoc(day8, part2)]
fn part2(tree_heigths: &Array2<u8>) -> u64 {
    let rows = tree_heigths.nrows();
    let cols = tree_heigths.ncols();

    let mut max_score = 0;
    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let val = tree_heigths[[i, j]];
            let distance_from_left = j;
            let distance_from_right = cols - j - 1;
            let distance_from_top = i;
            let distance_from_bottom = rows - i - 1;

            let visible_left = tree_heigths
                .slice(s![i, 0..j])
                .iter()
                .rev()
                .position(|height| *height >= val)
                .map(|x| x + 1)
                .unwrap_or(distance_from_left);

            let visible_right = tree_heigths
                .slice(s![i, (j + 1)..])
                .iter()
                .position(|height| *height >= val)
                .map(|x| x + 1)
                .unwrap_or(distance_from_right);

            let visible_top = tree_heigths
                .slice(s![0..i, j])
                .iter()
                .rev()
                .position(|height| *height >= val)
                .map(|x| x + 1)
                .unwrap_or(distance_from_top);

            let visible_down = tree_heigths
                .slice(s![(i + 1).., j])
                .iter()
                .position(|height| *height >= val)
                .map(|x| x + 1)
                .unwrap_or(distance_from_bottom);

            let scenic_score = (visible_left * visible_right * visible_top * visible_down) as u64;
            if scenic_score > max_score {
                max_score = scenic_score;
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    30373
    25512
    65332
    33549
    35390
    "};

    const PUZZLE_INPUT: &str = include_str!("../input/2022/day8.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 21);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(&parse_input(PUZZLE_INPUT)), 1681);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 8);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(&parse_input(PUZZLE_INPUT)), 201684);
    }
}
