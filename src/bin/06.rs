use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    const WINDOW_SIZE: usize = 4;
    let buffer: Vec<char> = input.chars().collect();
    for (i, window) in buffer.windows(WINDOW_SIZE).enumerate() {
        let unique = HashSet::<&char>::from_iter(window);
        if unique.len() == WINDOW_SIZE {
            return Some((WINDOW_SIZE + i) as u32);
        }
    }
    unreachable!();
}

pub fn part_two(input: &str) -> Option<u32> {
    const WINDOW_SIZE: usize = 14;
    let buffer: Vec<char> = input.chars().collect();
    for (i, window) in buffer.windows(WINDOW_SIZE).enumerate() {
        let unique = HashSet::<&char>::from_iter(window);
        if unique.len() == WINDOW_SIZE {
            return Some((WINDOW_SIZE + i) as u32);
        }
    }
    unreachable!();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
