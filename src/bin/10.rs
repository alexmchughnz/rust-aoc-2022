struct Event {
    val: i32,
    n_cycles: usize,
}
impl Event {
    fn tick(&mut self) -> Option<i32> {
        self.n_cycles -= 1;

        if self.n_cycles == 0 {
            let val = self.val;
            Some(val)
        } else {
            None
        }
    }
}

const NOOP: Event = Event {
    val: 0,
    n_cycles: 1,
};

fn addx(val: i32) -> Event {
    Event { val, n_cycles: 2 }
}

fn parse_events(input: &str) -> Vec<Event> {
    let mut events = Vec::<Event>::new();

    for mut items in input.lines().map(|l| l.split_whitespace()) {
        let _instr = items.next().unwrap();
        let event = match items.next() {
            Some(val) => addx(val.parse::<i32>().unwrap()),
            None => NOOP,
        };
        events.push(event);
    }

    events
}

pub fn part_one(input: &str) -> Option<i32> {
    const CYCLE_COUNTS: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut events = parse_events(input).into_iter();

    let mut rx = 1;
    let mut sum_strengths = 0;

    let mut queue = Vec::<Event>::new();
    let mut cycle = 0;

    loop {
        cycle += 1;
        if CYCLE_COUNTS.contains(&cycle) {
            sum_strengths += cycle as i32 * rx;
        }

        if queue.is_empty() {
            match events.next() {
                Some(e) => queue.push(e),
                None => break,
            }
        }

        for pending in queue.iter_mut() {
            if let Some(val) = pending.tick() {
                rx += val;
            }
        }

        queue.retain(|e| e.n_cycles > 0);
    }

    Some(sum_strengths)
}

pub fn part_two(input: &str) -> String {
    "".to_owned()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_owned()
        );
    }
}
