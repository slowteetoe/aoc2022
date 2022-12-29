use std::collections::BTreeMap;

use advent_of_code::helpers;
use priq::PriorityQueue;

// let's try something different, going to make a directed graph as we parse the grid
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    coord: (usize, usize),
    elevation: char,
    exits: Vec<(usize, usize)>,
    cost: usize, // since we have to move to a*, we'll use manhattan distance
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

pub fn reconstruct_path(
    came_from: BTreeMap<(usize, usize), (usize, usize)>,
    current: ((usize, usize), usize),
) -> Vec<(usize, usize)> {
    println!("GOOOOAAAAL!!!");
    let mut total_path = vec![];
    let mut curr = current.0;
    loop {
        total_path.push(curr);
        if curr == (0, 0) {
            break;
        }
        curr = *came_from.get(&curr).unwrap();
    }
    total_path.reverse();
    total_path
}

pub fn a_star(
    start: (usize, usize),
    goal: (usize, usize),
    map: &BTreeMap<(usize, usize), Node>,
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = PriorityQueue::new();
    let mut came_from = BTreeMap::<(usize, usize), (usize, usize)>::new();
    let mut g_score = BTreeMap::new();

    let start_node = map.get(&start).unwrap();

    open_set.put(start_node.coord, start_node.cost);
    g_score.insert(start_node.coord.clone(), 0);

    loop {
        if open_set.peek().is_none() {
            break;
        }
        let current = open_set.pop().unwrap(); // this removes from the PQ
        if current.0 == goal {
            return Some(reconstruct_path(came_from, current));
        }

        for neighbor in &map.get(&current.0).unwrap().exits {
            let tentative_g_score = g_score.get(&current.0).unwrap() + 1; // this is assuming d(current,neighbor) is one because they're all one hop away
            if tentative_g_score < *g_score.entry(*neighbor).or_insert(usize::MAX) {
                // this path to neighbor is better than any previous one, record it
                came_from.insert(*neighbor, current.0);
                g_score.insert(*neighbor, tentative_g_score);
                let neighbor_cost = &map.get(neighbor).unwrap().cost;
                let f = tentative_g_score + neighbor_cost;
                if open_set.iter().find(|k| k.0 == *neighbor).is_none() {
                    open_set.put(*neighbor, f);
                }
            }
        }
    }
    None
}

pub fn parse(input: &str) -> (BTreeMap<(usize, usize), Node>, (usize, usize)) {
    let mut h = BTreeMap::new();
    // going to make two passes through the data, just to make things easier - first will construct all the nodes, second will connect them
    let mut max_x = 0;
    let mut max_y = 0;
    let mut goal: Option<(usize, usize)> = None;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            if ch == 'E' {
                goal = Some((x, y));
            }
            let ch = match ch {
                'S' => 'a',
                'E' => 'z',
                _ => ch,
            };
            h.insert((x, y), Node::new((x, y), ch));
            max_x = x;
        });
        max_y = y;
    });
    let goal = goal.unwrap();

    // now that we know where the goal is, we can compute cost (manhattan distance) from each node, as well as track the connected nodes
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, _)| {
            let mut this_node = h.remove(&(x, y)).unwrap();
            // update the cost
            this_node.cost = helpers::manhattan_usize((x, y), (goal.0, goal.1));
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
    (h, goal)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, goal) = parse(input);

    let solution = a_star((0, 0), goal, &map);
    match solution {
        Some(p) => {
            // println!("path: {:?}", p);
            Some(p.len() - 1)
        }
        None => {
            println!("sad panda");
            None
        }
    }
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
