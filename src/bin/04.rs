use core::num;

fn is_subrange(range: (u32, u32), cand: (u32, u32)) -> bool {
    range.0 <= cand.0 && cand.1 <= range.1
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_overlapping: u32 = 0;
    for line in input.lines() {
        let mut nums = line.split(['-', ',']).map(|s| s.parse::<u32>().unwrap());
        let range_1 = (nums.next().unwrap(), nums.next().unwrap());
        let range_2 = (nums.next().unwrap(), nums.next().unwrap());

        if is_subrange(range_1, range_2) || is_subrange(range_2, range_1) {
            num_overlapping += 1;
        }
    }
    Some(num_overlapping)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
