use std::cell::RefCell;

struct Monkey<'a> {
    items: Vec<u32>,
    n_inspections: u32,
    inspect_fn: Box<dyn Fn(u32) -> u32 + 'a>,
    test_fn: Box<dyn Fn(&u32) -> usize + 'a>,
}
impl Monkey<'_> {
    fn inspect(&mut self, item: u32) -> u32 {
        let f = &self.inspect_fn;
        self.n_inspections += 1;
        f(item)
    }

    fn test(&self, item: &u32) -> usize {
        let f = &self.test_fn;
        f(item)
    }

    fn throw(target: &mut Self, item: u32) {
        target.items.push(item);
    }
}

impl Monkey<'_> {
    fn parse<'a>(desc: &str) -> Monkey {
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
            .map(|i| i.trim().parse::<u32>().unwrap())
            .collect();

        let mut operation_parts = operation_desc.split_whitespace().skip(1);
        let operation = parse_operation(operation_parts.next().unwrap());
        let operand = operation_parts.next().unwrap();
        let inspect = move |item: u32| -> u32 {
            match operand {
                "old" => operation(item, item),
                other => {
                    let operand = other.parse::<u32>().expect("Operand should be valid u32");
                    operation(item, operand)
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
            n_inspections: 0,
            inspect_fn: Box::new(inspect),
            test_fn: Box::new(test),
        }
    }
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = Vec::<RefCell<Monkey>>::new();
    for monkey_desc in input.split("\n\n") {
        monkeys.push(RefCell::new(Monkey::parse(monkey_desc)));
    }

    const NUM_ROUNDS: usize = 20;
    for _ in 0..NUM_ROUNDS {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            let items: Vec<_> = monkey.items.drain(..).collect();
            for mut item in items {
                item = monkey.inspect(item);
                item /= 3;
                let target_idx = monkey.test(&item);
                Monkey::throw(&mut monkeys[target_idx].borrow_mut(), item);
            }
        }
    }

    const N_ACTIVE: usize = 2;
    monkeys.sort_by_key(|m| m.borrow().n_inspections);
    let most_active = monkeys.into_iter().rev().take(N_ACTIVE);
    let monkey_business: u32 = most_active.map(|m| m.borrow().n_inspections).product();

    Some(monkey_business)
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
