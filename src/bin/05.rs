type CrateStructure = Vec<Vec<char>>;
type Move = (u32, usize, usize);
enum MoveStrategy {
    Single,
    Multiple,
}

fn read_crates(drawing: &str) -> CrateStructure {
    const CRATE_OFFSET: usize = 1;
    const CRATE_WIDTH: usize = 4;

    let mut lines: Vec<&str> = drawing.split("\n").collect();

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

fn read_moves(procedure: &str) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();
    for line in procedure.lines() {
        let mut nums_iter = line
            .split_whitespace()
            .filter_map(|w| w.parse::<usize>().ok());
        let nums = (
            nums_iter.next().unwrap() as u32, // Number of crates to move
            nums_iter.next().unwrap() - 1,    // Index of src stack
            nums_iter.next().unwrap() - 1,    // Index of dest stack
        );
        moves.push(nums);
    }
    moves
}

fn perform_moves(crates: &mut CrateStructure, moves: Vec<Move>, strategy: MoveStrategy) {
    for (num, src, dest) in moves {
        match strategy {
            MoveStrategy::Single => {
                for _ in 0..num {
                    let moved = crates[src].pop().expect("Should not move from empty stack");
                    crates[dest].push(moved);
                }
            }
            MoveStrategy::Multiple => {
                let len = crates[src].len();
                let range_to_move = (len - num as usize)..(len);
                let mut moved: Vec<_> = crates[src].drain(range_to_move).collect();
                crates[dest].append(&mut moved);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut info = input.split("\n\n");
    let (drawing, procedure) = (info.next().unwrap(), info.next().unwrap());

    let mut crates = read_crates(drawing);
    let moves = read_moves(procedure);
    perform_moves(&mut crates, moves, MoveStrategy::Single);

    let top_crates = crates
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect();

    Some(top_crates)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut info = input.split("\n\n");
    let (drawing, procedure) = (info.next().unwrap(), info.next().unwrap());

    let mut crates = read_crates(drawing);
    let moves = read_moves(procedure);
    perform_moves(&mut crates, moves, MoveStrategy::Multiple);

    let top_crates = crates
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect();

    Some(top_crates)
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
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
