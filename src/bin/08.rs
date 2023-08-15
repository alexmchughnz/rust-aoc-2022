type Forest = Box<[Box<[u8]>]>;

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

fn is_highest(tree: &u8, line: &Vec<&u8>) -> bool {
    line.into_iter().all(|t| tree > *t)
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = parse_forest(input);

    let n_cols = forest.len();
    let n_rows = forest[0].len();

    let mut n_visible = n_cols * 2 + (n_rows - 2) * 2; // Outside edges are already visible

    // Iterate over inner trees.
    for r in 1..n_rows - 1 {
        let row = &forest[r];
        for c in 1..n_cols - 1 {
            let height = &row[c];
            let above = forest[0..r].iter().map(|row| &row[c]).collect::<Vec<_>>();
            let below = forest[r + 1..n_rows]
                .iter()
                .map(|row| &row[c])
                .collect::<Vec<_>>();
            let left = forest[r][0..c].iter().collect::<Vec<_>>();
            let right = forest[r][c + 1..n_cols].iter().collect::<Vec<_>>();

            if [above, below, left, right]
                .iter()
                .any(|seq| is_highest(height, seq))
            {
                n_visible += 1;
            }
        }
    }

    Some(n_visible as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
