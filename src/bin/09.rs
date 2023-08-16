use std::collections::HashSet;

#[derive(Hash, Default, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

struct Move {
    dir: Direction,
    num: u32,
}

impl Position {
    fn is_adjacent(&self, other: &Self) -> bool {
        let x_adj = (self.x - other.x).abs() <= 1;
        let y_adj = (self.y - other.y).abs() <= 1;
        x_adj && y_adj
    }

    fn move_once(&mut self, dir: &Direction) {
        match dir {
            Up => self.y += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
    }

    fn move_towards(&mut self, other: &Self) {
        self.x += (other.x - self.x).signum();
        self.y += (other.y - self.y).signum();
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();
    for mut line in input.lines().map(|l| l.split_whitespace()) {
        let dir = match line.next().unwrap() {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Only four valid directions."),
        };
        let num = line.next().unwrap().parse::<u32>().unwrap();
        moves.push(Move { dir, num });
    }

    moves
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Position {
        ..Default::default()
    };
    let mut tail = Position {
        ..Default::default()
    };

    let mut tail_visited: HashSet<Position> = HashSet::from([tail.clone()]);

    for m in parse_moves(input) {
        for _ in 0..m.num {
            head.move_once(&m.dir);
            if !tail.is_adjacent(&head) {
                tail.move_towards(&head);
                tail_visited.insert(tail.clone());
            }
        }
    }

    Some(tail_visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    const NUM_KNOTS: usize = 10;
    let mut knots: [Position; NUM_KNOTS] = Default::default();

    let mut tail_visited: HashSet<Position> = HashSet::from([knots.last().unwrap().clone()]);

    for m in parse_moves(input) {
        for _ in 0..m.num {
            knots.first_mut().unwrap().move_once(&m.dir);
            for i in 1..knots.len() {
                let (ahead, remaining) = knots.split_at_mut(i);
                let knot = remaining.first_mut().unwrap();
                let leader = ahead.last().unwrap();

                if !knot.is_adjacent(leader) {
                    knot.move_towards(leader);
                }
            }

            tail_visited.insert(knots.last().unwrap().clone());
        }
    }

    Some(tail_visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
