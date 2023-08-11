#[derive(Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;
const SHAPE_ORDER: [Shape; 3] = [Rock, Paper, Scissors]; // Higher index beats lower index, e.g. Paper > Rock

#[derive(Clone, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
use Outcome::*;

fn elf_char_to_shape(char: &str) -> Shape {
    match char {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn my_char_to_shape(char: &str) -> Shape {
    match char {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn my_char_to_outcome(char: &str) -> Outcome {
    match char {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
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

fn determine_outcome_for_game(elf_shape: &Shape, my_shape: &Shape) -> Outcome {
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

fn determine_shape_for_outcome(elf_shape: &Shape, outcome: &Outcome) -> Shape {
    match outcome {
        Draw => elf_shape.clone(),
        Win => SHAPE_ORDER
            .into_iter()
            .cycle()
            .skip_while(|s| s != elf_shape)
            .skip(1) // Elf's shape
            .next() // Winner vs elf's shape
            .expect("cycle iteration should never end"),
        Lose => SHAPE_ORDER
            .into_iter()
            .rev()
            .cycle()
            .skip_while(|s| s != elf_shape)
            .skip(1) // Elf's shape
            .next() // Loser vs elf's shape
            .expect("cycle iteration should never end"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for mut line in input.lines().map(|l| l.split_whitespace()) {
        let elf_shape = elf_char_to_shape(line.next().unwrap());
        let my_shape = my_char_to_shape(line.next().unwrap());

        score += score_shape(&my_shape);
        score += score_outcome(&determine_outcome_for_game(&elf_shape, &my_shape));
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for mut line in input.lines().map(|l| l.split_whitespace()) {
        let elf_shape = elf_char_to_shape(line.next().unwrap());
        let outcome = my_char_to_outcome(line.next().unwrap());

        let my_shape = determine_shape_for_outcome(&elf_shape, &outcome);

        score += score_shape(&my_shape);
        score += score_outcome(&outcome);
    }
    Some(score)
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
        assert_eq!(part_two(&input), Some(12));
    }
}
