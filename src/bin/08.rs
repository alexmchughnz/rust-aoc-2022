use std::collections::HashMap;

type Forest = Box<[Box<[u8]>]>;
fn size(forest: &Forest) -> (usize, usize) {
    let n_rows = forest.len();
    let n_cols = forest[0].len();
    (n_rows, n_cols)
}

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use std::ops::Range;

use Direction::*;

fn parse_forest(input: &str) -> Forest {
    let mut forest = Vec::<Box<[u8]>>::new();

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            .into_boxed_slice();
        forest.push(row);
    }

    forest.into_boxed_slice()
}

fn surrounding_heights((r, c): (usize, usize), forest: &Forest) -> HashMap<Direction, Vec<&u8>> {
    let (n_rows, n_cols) = size(forest);
    let horz = |r: usize, cs: Range<usize>| forest[r][cs].iter();
    let vert = |rs: Range<usize>, c: usize| forest[rs].iter().map(move |row| &row[c]);

    HashMap::from([
        (Up, vert(0..r, c).rev().collect()),
        (Down, vert(r + 1..n_rows, c).collect()),
        (Left, horz(r, 0..c).rev().collect()),
        (Right, horz(r, c + 1..n_cols).collect()),
    ])
}

fn is_highest(tree: &u8, line: Vec<&u8>) -> bool {
    line.into_iter().all(|t| tree > t)
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = parse_forest(input);
    let (n_rows, n_cols) = size(&forest);

    let mut n_visible = n_cols * 2 + (n_rows - 2) * 2; // Outside edges are already visible

    // Iterate over inner trees.
    for r in 1..n_rows - 1 {
        let row = &forest[r];
        for c in 1..n_cols - 1 {
            let height = &row[c];

            let surrounding = surrounding_heights((r, c), &forest);
            if surrounding.into_values().any(|seq| is_highest(height, seq)) {
                n_visible += 1;
            }
        }
    }

    Some(n_visible as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = parse_forest(input);
    let (n_rows, n_cols) = size(&forest);

    let mut best_score: u32 = 0;
    // Iterate over inner trees.
    for r in 1..n_rows - 1 {
        let row = &forest[r];
        for c in 1..n_cols - 1 {
            let height = &row[c];

            let surrounding = surrounding_heights((r, c), &forest);
            let scores_by_direction = surrounding.into_values().map(|dir| {
                match dir.iter().position(|t| *t >= height) {
                    Some(idx) => idx + 1, // Num of trees until a blocking one (incl.)
                    None => dir.len(),    // No blocking trees, count all in that direction
                }
            });

            let score = scores_by_direction.product::<usize>() as u32;
            if score > best_score {
                best_score = score;
            }
        }
    }
    Some(best_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
