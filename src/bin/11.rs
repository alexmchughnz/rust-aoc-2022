use std::str::FromStr;

struct Monkey {
    items: Vec<u32>,
    inspect: Box<dyn FnMut(&mut u32)>,
    test: Box<dyn Fn(&u32) -> usize>,
}
impl Monkey {
    fn throw(&mut self, item: &u32, target: &mut Self) {}
}

fn parse_operation(symbol: &str) -> impl Fn(u32, u32) -> u32 {
    match symbol {
        "*" => |a, b| a * b,
        "+" => |a, b| a + b,
        _ => panic!("Attempt to parse invalid operation"),
    }
}

fn parse_ending_number(string: &str) -> u32 {
    string
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn parse_monkey(desc: &str) -> Monkey {
    let mut lines = desc
        .lines()
        .map(move |l: &str| l.split(':').last().unwrap());
    lines.next();
    let items_desc = lines.next().unwrap();
    let operation_desc = lines.next().unwrap().split('=').last().unwrap();
    let test_desc = lines.next().unwrap();
    let true_desc = lines.next().unwrap();
    let false_desc = lines.next().unwrap();

    let items = items_desc
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    let mut operation_parts = operation_desc.split_whitespace().skip(1);
    let operation = parse_operation(operation_parts.next().unwrap());
    let inspect = move |item: &mut u32| {
        *item = match operation_parts.next().unwrap() {
            "old" => operation(*item, *item),
            other => {
                let operand = other.parse::<u32>().expect("Operand should be valid u32");
                operation(*item, operand)
            }
        }
    };

    let divisor = parse_ending_number(test_desc);
    let true_target = parse_ending_number(true_desc) as usize;
    let false_target = parse_ending_number(false_desc) as usize;
    let test = move |item: &u32| {
        if item % divisor == 0 {
            true_target
        } else {
            false_target
        }
    };

    Monkey {
        items,
        inspect: Box::new(inspect),
        test: Box::new(test),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = Vec::<Monkey>::new();
    for monkey_desc in input.split("\n\n") {
        monkeys.push(parse_monkey(monkey_desc));
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
