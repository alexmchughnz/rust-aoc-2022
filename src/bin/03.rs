use std::collections::HashSet;

fn item_to_priority(item: &char) -> u32 {
    let mut priority = 1 + item.to_ascii_lowercase() as u32 - 'a' as u32;
    if item.is_ascii_uppercase() {
        priority += 26;
    }
    priority
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_priorities: u32 = 0;
    for line in input.lines() {
        let rucksack_a = &line[..line.len() / 2];
        let rucksack_b = &line[line.len() / 2..];
        let items_a: HashSet<char> = HashSet::from_iter(rucksack_a.chars());
        let items_b: HashSet<char> = HashSet::from_iter(rucksack_b.chars());

        let duplicate = items_a
            .intersection(&items_b)
            .next()
            .expect("Rucksacks should share one item");
        sum_priorities += item_to_priority(duplicate);
    }
    Some(sum_priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
