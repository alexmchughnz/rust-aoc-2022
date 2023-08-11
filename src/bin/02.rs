#[derive(Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;
const SHAPE_ORDER: [Shape; 3] = [Rock, Paper, Scissors]; // Higher index beats lower index, e.g. Paper > Rock

enum Outcome {
    Win,
    Lose,
    Draw,
}
use Outcome::*;

fn map_elf_char(char: &str) -> Shape {
    match char {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn map_my_char(char: &str) -> Shape {
    match char {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn score_shape(shape: &Shape) -> u32 {
    match shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn score_outcome(outcome: &Outcome) -> u32 {
    match outcome {
        Win => 6,
        Draw => 3,
        Lose => 0,
    }
}

fn get_outcome_for_game(elf_shape: &Shape, my_shape: &Shape) -> Outcome {
    if elf_shape == my_shape {
        return Draw;
    }

    let my_winner = SHAPE_ORDER
        .into_iter()
        .cycle()
        .skip_while(|s| s != elf_shape)
        .skip(1) // Elf's shape
        .next() // Winner vs elf's shape
        .expect("cycle iteration should never end");
    if *my_shape == my_winner {
        return Win;
    } else {
        return Lose;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for mut line in input.lines().map(|l| l.split_whitespace()) {
        let elf_shape = map_elf_char(line.next().unwrap());
        let my_shape = map_my_char(line.next().unwrap());

        score += score_shape(&my_shape);
        score += score_outcome(&get_outcome_for_game(&elf_shape, &my_shape));
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
