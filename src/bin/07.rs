use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::*;

type DirPointer = Rc<RefCell<Dir>>;

enum ChildNode {
    DirPointer(DirPointer),
    File(u32),
}

struct Dir {
    parent: Option<DirPointer>,
    children: HashMap<String, ChildNode>,
}
impl Dir {
    fn new_pointer(parent: Option<DirPointer>) -> DirPointer {
        let new_dir = Dir {
            parent,
            children: HashMap::new(),
        };

        Rc::new(RefCell::new(new_dir))
    }
}

fn execute_cd(curr: DirPointer, root: DirPointer, cd_cmd: &str) -> DirPointer {
    let dest = cd_cmd
        .split_whitespace()
        .last()
        .expect("`cd` command should specify dest dir");

    return match dest {
        "/" => root,

        ".." => curr
            .borrow()
            .parent
            .clone()
            .expect("Can only go up a level if dir has parent"),

        other => {
            let child = &curr.borrow().children[other];
            if let ChildNode::DirPointer(dir) = child {
                dir.clone()
            } else {
                panic!("Cannot `cd` into file")
            }
        }
    };
}

fn execute_ls<'a>(curr: DirPointer, lines: impl Iterator<Item = &'a str>) {
    for line in lines {
        let mut data = line.split_whitespace();
        let (kind, name) = (data.next().unwrap(), data.next().unwrap());
        let child = match kind.parse::<u32>() {
            Ok(size) => ChildNode::File(size),
            _ => {
                let new_dir = Dir::new_pointer(Some(curr.clone()));
                ChildNode::DirPointer(new_dir)
            }
        };

        curr.borrow_mut().children.insert(String::from(name), child);
    }
}

fn parse_tree(input: &str) -> DirPointer {
    let mut lines = input.lines().peekable();

    // Create root node.
    let root = Dir::new_pointer(None);
    let mut curr = root.clone();

    while let Some(cmd) = lines.next() {
        if cmd.contains("cd") {
            curr = execute_cd(curr, root.clone(), cmd);
        } else if cmd.contains("ls") {
            let contents = std::iter::from_fn(|| lines.next_if(|l| !l.starts_with('$')));
            execute_ls(curr.clone(), contents);
        } else {
            panic!("Invalid command");
        }
    }
    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = parse_tree(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
