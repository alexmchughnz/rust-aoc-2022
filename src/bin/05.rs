type CrateStructure = Vec<Vec<char>>;

fn read_crates(state: &str) -> CrateStructure {
    const CRATE_OFFSET: usize = 1;
    const CRATE_WIDTH: usize = 4;

    let mut lines: Vec<&str> = state.split("\n").collect();

    let num_stacks = lines.pop().unwrap().split_whitespace().count();
    let mut structure: CrateStructure = vec![Vec::<char>::new(); num_stacks];

    for line in lines.into_iter().rev() {
        let crates = line.chars().skip(CRATE_OFFSET).step_by(CRATE_WIDTH);
        for (i, item) in crates.enumerate() {
            if item != ' ' {
                structure[i].push(item);
            }
        }
    }
    structure
}

pub fn part_one(input: &str) -> Option<String> {
    let mut info = input.split("\n\n");
    let (state, moves) = (info.next().unwrap(), info.next().unwrap());
    let crates = read_crates(state);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
