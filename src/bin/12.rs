use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Display,
    rc::Rc,
};

use itertools::Itertools;

// let's try something different, going to make a directed graph as we parse the grid
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    coord: (usize, usize),
    elevation: char, // mostly for debugging, not like I ever need that =/
    exits: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(coord: (usize, usize), elevation: char) -> Self {
        Self {
            coord,
            elevation,
            exits: vec![],
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("coord", &self.coord)
            .field("elevation", &self.elevation)
            .field(
                "exits",
                &self
                    .exits
                    .iter()
                    .map(|n| {
                        let cell = Borrow::<RefCell<Node>>::borrow(n).borrow().coord;
                        format!("{:?}", cell)
                    })
                    .collect_vec()
                    .join(","),
            )
            .finish()
    }
}

pub fn walk_it(curr: &Rc<RefCell<Node>>, path: &mut BTreeSet<(usize, usize)>) -> Option<usize> {
    let this_node = Borrow::<RefCell<Node>>::borrow(curr).borrow();
    if this_node.elevation == 'E' {
        println!("At target, found our path, {:?}", &path);
        return Some(path.len());
    }
    let pos = this_node.coord;
    println!("Currently at {:?}, path so far: {:?}", &pos, &path);
    for opt in this_node.exits.iter() {
        println!("looking at {:?}", opt);
        if !path.contains(&Borrow::<RefCell<Node>>::borrow(opt).borrow().coord) {
            println!("Taking path option: {:?}", opt);
            path.insert(pos.clone());
            let result = walk_it(opt, path);
            match result {
                Some(n) => {
                    if n < 33 {
                        return Some(n);
                    }
                }
                _ => {}
            }
            println!("have to backtrack");
            path.remove(&pos.clone());
        }
    }
    None
}

pub fn parse(input: &str) -> Rc<RefCell<Node>> {
    // if we hold onto the starting node, every node that we need to traverse will be attached, since there's a path to the exit
    let mut h = BTreeMap::new();
    // going to make two passes through the data, just to make things easier - first will construct all the nodes, second will connect them
    let mut max_x = 0;
    let mut max_y = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let ch = match ch {
                'S' => 'a',
                // 'E' => 'z',
                _ => ch,
            };
            h.insert((x, y), Rc::new(RefCell::new(Node::new((x, y), ch))));
            max_x = x;
        });
        max_y = y;
    });
    println!("dim: ({:?},{:?})", max_x, max_x);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, _)| {
            let this_node = h.get(&(x, y)).unwrap();
            // check the cardinals
            if y > 0 {
                let n = h.get(&(x, y - 1)).unwrap();
                if n.borrow_mut().elevation as u8
                    <= Borrow::<RefCell<Node>>::borrow(this_node)
                        .borrow()
                        .elevation as u8
                        + 1
                {
                    this_node.borrow_mut().exits.push(Rc::clone(n));
                }
            }
            if y < max_y {
                let n = h.get(&(x, y + 1)).unwrap();
                if n.borrow_mut().elevation as u8
                    <= Borrow::<RefCell<Node>>::borrow(this_node)
                        .borrow()
                        .elevation as u8
                        + 1
                {
                    this_node.borrow_mut().exits.push(Rc::clone(n));
                }
            }
            if x > 0 {
                let n = h.get(&(x - 1, y)).unwrap();
                if n.borrow_mut().elevation as u8
                    <= Borrow::<RefCell<Node>>::borrow(this_node)
                        .borrow()
                        .elevation as u8
                        + 1
                {
                    this_node.borrow_mut().exits.push(Rc::clone(n));
                }
            }
            if x < max_x {
                let n = h.get(&(x + 1, y)).unwrap();
                if n.borrow_mut().elevation as u8
                    <= Borrow::<RefCell<Node>>::borrow(this_node)
                        .borrow()
                        .elevation as u8
                        + 1
                {
                    this_node.borrow_mut().exits.push(Rc::clone(n));
                }
            }
        });
    });
    println!("{:#?}", h);
    h.remove(&(0, 0)).unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let root = parse(input);
    println!("{:?}", root);
    let mut path = BTreeSet::<(usize, usize)>::new();
    let answer = walk_it(&root, &mut path);
    Some(answer.unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
