use std::cell::RefCell;

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}
use Operation::*;

struct Monkey {
    items: Vec<u64>,
    n_inspections: u64,
    operation: Operation,
    divisor: u64,
    true_idx: usize,
    false_idx: usize,
}
impl Monkey {
    fn inspect(&mut self, item: &mut u64) {
        self.n_inspections += 1;

        *item = match self.operation {
            Add(addend) => *item + addend,
            Multiply(multiplier) => *item * multiplier,
            Square => *item * *item,
        }
    }

    fn test(&self, item: &u64) -> usize {
        if item % self.divisor == 0 {
            self.true_idx
        } else {
            self.false_idx
        }
    }

    fn throw(target: &mut Self, item: u64) {
        target.items.push(item);
    }
}

impl Monkey {
    fn parse(desc: &str) -> Monkey {
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
            .map(|i| i.trim().parse::<u64>().unwrap())
            .collect();

        let mut operation_parts = operation_desc.split_whitespace().skip(1);
        let symbol = operation_parts.next().unwrap().trim();
        let operand = operation_parts.next().unwrap().trim();
        let operation = if operand == "old" {
            Square
        } else {
            match symbol {
                "*" => Multiply(operand.parse::<u64>().unwrap()),
                "+" => Add(operand.parse::<u64>().unwrap()),
                _ => panic!("Attempt to parse invalid operation symbol"),
            }
        };

        let divisor = parse_ending_number(test_desc);
        let true_idx = parse_ending_number(true_desc) as usize;
        let false_idx = parse_ending_number(false_desc) as usize;

        Monkey {
            items,
            n_inspections: 0,
            operation,
            divisor,
            true_idx,
            false_idx,
        }
    }
}

fn parse_ending_number(string: &str) -> u64 {
    string
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = Vec::<RefCell<Monkey>>::new();
    for monkey_desc in input.split("\n\n") {
        monkeys.push(RefCell::new(Monkey::parse(monkey_desc)));
    }

    const NUM_ROUNDS: usize = 20;
    for _ in 0..NUM_ROUNDS {
        for mut monkey in monkeys.iter().map(|m| m.borrow_mut()) {
            let mut items: Vec<_> = monkey.items.drain(..).collect();
            for item in items.iter_mut() {
                monkey.inspect(item);
                *item /= 3;
                let target_idx = monkey.test(item);
                Monkey::throw(&mut monkeys[target_idx].borrow_mut(), *item);
            }
        }
    }

    const N_ACTIVE: usize = 2;
    monkeys.sort_by_key(|m| m.borrow().n_inspections);
    let most_active = monkeys.into_iter().rev().take(N_ACTIVE);
    let monkey_business: u64 = most_active.map(|m| m.borrow().n_inspections).product();

    Some(monkey_business)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = Vec::<RefCell<Monkey>>::new();
    for monkey_desc in input.split("\n\n") {
        monkeys.push(RefCell::new(Monkey::parse(monkey_desc)));
    }
    let common_divisor: u64 = monkeys.iter().map(|m| m.borrow().divisor).product();

    const NUM_ROUNDS: usize = 10_000;
    for _ in 0..NUM_ROUNDS {
        for mut monkey in monkeys.iter().map(|m| m.borrow_mut()) {
            let mut items: Vec<_> = monkey.items.drain(..).collect();
            for item in items.iter_mut() {
                monkey.inspect(item);
                *item %= common_divisor;
                let target_idx = monkey.test(item);
                Monkey::throw(&mut monkeys[target_idx].borrow_mut(), *item);
            }
        }
    }

    const N_ACTIVE: usize = 2;
    monkeys.sort_by_key(|m| m.borrow().n_inspections);
    let most_active = monkeys.into_iter().rev().take(N_ACTIVE);
    let monkey_business: u64 = most_active.map(|m| m.borrow().n_inspections).product();

    Some(monkey_business)
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
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
