use std::collections::{BTreeMap, BTreeSet};

use advent_of_code::helpers;
use priority_queue::PriorityQueue;

// let's try something different, going to make a directed graph as we parse the grid
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    coord: (usize, usize),
    elevation: char, // mostly for debugging, not like I ever need that =/
    exits: Vec<(usize, usize)>,
    cost: usize, // since we have to move to a* or dijkstra, we'll just use manhattan distance
}

impl Node {
    pub fn new(coord: (usize, usize), elevation: char) -> Self {
        Self {
            coord,
            elevation,
            exits: vec![],
            cost: usize::MAX,
        }
    }
}

// impl std::fmt::Debug for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Node")
//             .field("coord", &self.coord)
//             .field("elevation", &self.elevation)
//             .field("cost", &self.cost)
//             .field(
//                 "exits",
//                 &self
//                     .exits
//                     .iter()
//                     .map(|n| {
//                         let cell = Borrow::<RefCell<Node>>::borrow(n).borrow().coord;
//                         format!("{:?}", cell)
//                     })
//                     .collect_vec()
//                     .join(","),
//             )
//             .finish()
//     }
// }

// FIXME implement a*
// pub fn reconstruct_path(
//     came_from: BTreeMap<(usize, usize), (usize, usize)>,
//     current: (&(usize,usize), &usize),
// ) -> Vec<(usize, usize)> {
//     let mut total_path = vec![*current.0];
//     for cur in came_from.keys() {
//         total_path.push((cur.0, cur.1));
//     }
//     total_path
// }

// pub fn a_star(start: &Rc<RefCell<Node>>, goal: &Rc<RefCell<Node>>, h) -> Option<Vec<(usize, usize)>>{
//     let mut open_set = PriorityQueue::new();
//     let s = Borrow::<RefCell<Node>>::borrow(start).borrow();
//     open_set.push(s.coord, s.cost);
//     let came_from = BTreeMap::<(usize,usize), Rc<RefCell<Node>>>::new();
//     let mut g_score = BTreeMap::new();
//     g_score.insert(s.coord.clone(), 0);
//     let mut f_score = BTreeMap::new();
//     f_score.insert(s.coord, s.cost);

//     while !open_set.is_empty() {
//         let current = open_set.peek().unwrap();
//         if *current.0 == Borrow::<RefCell<Node>>::borrow(goal).borrow().coord {
//             return Some(reconstruct_path(came_from, current));
//         }
//         open_set.remove(current.0);
//         for neighbor in
//     }
//     None
// }

pub fn walk_it(
    curr: &Node,
    path: &mut BTreeSet<(usize, usize)>,
    map: &BTreeMap<(usize, usize), Node>,
) -> Option<usize> {
    let this_node = curr;
    if this_node.elevation == 'E' {
        println!("At target, found our path, {:?}", &path);
        return Some(path.len());
    }
    let pos = this_node.coord;
    println!("Currently at {:?}, path so far: {:?}", &pos, &path);
    for opt in this_node.exits.iter() {
        println!("looking at {:?}", opt);
        if !path.contains(opt) {
            println!("Taking path option: {:?}", opt);
            path.insert(pos.clone());
            let result = walk_it(map.get(opt).unwrap(), path, map);
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

pub fn parse(input: &str) -> BTreeMap<(usize, usize), Node> {
    // if we hold onto the starting node, every node that we need to traverse will be attached, since there's a path to the exit
    let mut h = BTreeMap::new();
    // going to make two passes through the data, just to make things easier - first will construct all the nodes, second will connect them
    let mut max_x = 0;
    let mut max_y = 0;
    let mut goal: Option<(usize, usize)> = None;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let ch = match ch {
                'S' => 'a',
                // 'E' => 'z',
                _ => ch,
            };
            if ch == 'E' {
                goal = Some((x, y));
            }
            h.insert((x, y), Node::new((x, y), ch));
            max_x = x;
        });
        max_y = y;
    });
    let goal = goal.unwrap();
    println!("dim: ({:?},{:?}) with goal at: {:?}", max_x, max_x, goal);

    // now that we know where the goal is, we can compute cost (manhattan distance) from each node, as well as track the connected nodes
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, _)| {
            let mut this_node = h.remove(&(x, y)).unwrap();
            // update the cost
            this_node.cost = helpers::manhattan_usize((x, y), (goal.0, goal.1));
            // check the cardinals
            if y > 0 {
                let n = h.get(&(x, y - 1)).unwrap();
                if n.elevation as u8 <= this_node.elevation as u8 + 1 {
                    this_node.exits.push(n.coord);
                }
            }
            if y < max_y {
                let n = h.get(&(x, y + 1)).unwrap();
                if n.elevation as u8 <= this_node.elevation as u8 + 1 {
                    this_node.exits.push(n.coord);
                }
            }
            if x > 0 {
                let n = h.get(&(x - 1, y)).unwrap();
                if n.elevation as u8 <= this_node.elevation as u8 + 1 {
                    this_node.exits.push(n.coord);
                }
            }
            if x < max_x {
                let n = h.get(&(x + 1, y)).unwrap();
                if n.elevation as u8 <= this_node.elevation as u8 + 1 {
                    this_node.exits.push(n.coord);
                }
            }
            h.insert((x, y), this_node);
        });
    });
    println!("{:#?}", h);
    h
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    println!("{:?}", &map);
    let root = map.get(&(0, 0)).unwrap();
    let mut path = BTreeSet::<(usize, usize)>::new();
    let answer = walk_it(root, &mut path, &map);
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
