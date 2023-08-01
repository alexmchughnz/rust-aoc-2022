pub fn part_one(input: &str) -> Option<u32> {
    let elves = input.split("\n\n");

    let calories = elves.map(|elf| {
        elf.trim()
            .split('\n')
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    calories.max()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
