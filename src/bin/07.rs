use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::*;

type DirPointer = Rc<RefCell<Dir>>;

enum Node {
    DirPointer(DirPointer),
    File(u32),
}
impl Node {
    fn size(&self) -> u32 {
        match self {
            Node::File(size) => *size,
            Node::DirPointer(p) => {
                let mut dir = p.borrow_mut();
                if dir.size.is_none() {
                    dir.size = Some(dir.children.values().map(|n| n.size()).sum());
                }
                dir.size.expect("Dir should have a computed size")
            }
        }
    }
}

struct Dir {
    parent: Option<DirPointer>,
    children: HashMap<String, Node>,
    size: Option<u32>,
}
impl Dir {
    fn new_pointer(parent: Option<DirPointer>) -> DirPointer {
        let new_dir = Dir {
            parent,
            children: HashMap::new(),
            size: None,
        };

        Rc::new(RefCell::new(new_dir))
    }
}

fn execute_cd(curr: DirPointer, root: DirPointer, cd_cmd: &str) -> DirPointer {
    let dest = cd_cmd
        .split_whitespace()
        .last()
        .expect("`cd` command should specify dest dir");

    let new_dir = match dest {
        "/" => root,

        ".." => curr
            .borrow()
            .parent
            .clone()
            .expect("Can only go up a level if dir has parent"),

        other => {
            let child = &curr.borrow().children[other];
            if let Node::DirPointer(dir) = child {
                dir.clone()
            } else {
                panic!("Cannot `cd` into file")
            }
        }
    };

    new_dir
}

fn execute_ls<'a>(curr: DirPointer, lines: impl Iterator<Item = &'a str>) {
    for line in lines {
        let mut data = line.split_whitespace();
        let (kind, name) = (data.next().unwrap(), data.next().unwrap());
        let child = match kind.parse::<u32>() {
            Ok(size) => Node::File(size),
            _ => {
                let new_dir = Dir::new_pointer(Some(curr.clone()));
                Node::DirPointer(new_dir)
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

fn traverse_sizes(node: &Node) -> Vec<u32> {
    let mut sizes = Vec::<u32>::new();
    if let Node::DirPointer(p) = node {
        sizes.push(node.size());
        let mut child_sizes: Vec<u32> = p
            .borrow()
            .children
            .values()
            .flat_map(|p| traverse_sizes(p))
            .collect();
        sizes.append(&mut child_sizes);
    }
    sizes
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = parse_tree(input);
    let dir_sizes = traverse_sizes(&Node::DirPointer(tree));
    let deletable_size: u32 = dir_sizes.into_iter().filter(|s| *s <= 100000).sum();
    Some(deletable_size)
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
