pub fn part_one(input: &str) -> Option<u32> {
    let elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|cal| cal.parse::<u32>().unwrap()).sum())
        .collect();

    elves.into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|cal| cal.parse::<u32>().unwrap()).sum())
        .collect();

    elves.sort();

    Some(elves.into_iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
