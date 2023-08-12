fn fully_contains(a: (u32, u32), b: (u32, u32)) -> bool {
    let range_a = a.0..=a.1;
    let start_in_a = range_a.contains(&b.0);
    let end_in_a = range_a.contains(&b.1);

    start_in_a && end_in_a
}

fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
    let range_a = a.0..=a.1;
    let start_in_a = range_a.contains(&b.0);
    let end_in_a = range_a.contains(&b.1);

    start_in_a || end_in_a
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_fully_contained: u32 = 0;
    for line in input.lines() {
        let mut nums = line.split(['-', ',']).map(|s| s.parse::<u32>().unwrap());
        let range_1 = (nums.next().unwrap(), nums.next().unwrap());
        let range_2 = (nums.next().unwrap(), nums.next().unwrap());

        if fully_contains(range_1, range_2) || fully_contains(range_2, range_1) {
            num_fully_contained += 1;
        }
    }
    Some(num_fully_contained)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_overlapping: u32 = 0;
    for line in input.lines() {
        let mut nums = line.split(['-', ',']).map(|s| s.parse::<u32>().unwrap());
        let range_1 = (nums.next().unwrap(), nums.next().unwrap());
        let range_2 = (nums.next().unwrap(), nums.next().unwrap());

        if contains(range_1, range_2) || contains(range_2, range_1) {
            num_overlapping += 1;
        }
    }
    Some(num_overlapping)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
