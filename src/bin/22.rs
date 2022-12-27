use std::{cmp, collections::BTreeMap};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Entity(char);

// TIL about associated constants
impl Entity {
    const NONE: Self = Self(' ');
    const WALL: Self = Self('#');
    const SPACE: Self = Self('.');
}

#[derive(Debug, PartialEq, Eq)]
pub struct Facing(char, usize);
impl Facing {
    const RIGHT: Self = Self('>', 0);
    const DOWN: Self = Self('V', 1);
    const LEFT: Self = Self('<', 2);
    const UP: Self = Self('^', 3);
}

#[derive(Debug)]
pub struct Player {
    pos: (usize, usize),
    facing: Facing,
    instructions: Vec<String>, // e.g. 10R, 5L
    map: BTreeMap<(usize, usize), Entity>,
    max_x: usize, // helpful for display
}
impl Player {
    pub fn new(map: BTreeMap<(usize, usize), Entity>, max_x: usize, instructions: &str) -> Self {
        // initial pos = leftmost cell of top row in map
        let x_pos = map
            .iter()
            .map(|(k, v)| match (k.1, v) {
                (0, &Entity::SPACE) => k.0,
                _ => usize::MAX,
            })
            .min()
            .unwrap();
        println!("Initial pos: ({x_pos}, 0)");
        Self {
            pos: ({ x_pos }, 0),
            facing: Facing::RIGHT,
            instructions: Self::parse_instructions(instructions),
            map,
            max_x,
        }
    }

    pub fn parse_instructions(input: &str) -> Vec<String> {
        // e.g. 10R5L5R10L4R5L5
        let re = Regex::new(r"(\d+[UDLR]{1})+?").unwrap();
        let mut instr = re
            .captures_iter(input)
            .map(|c| c.get(1).unwrap().as_str().to_string())
            .collect_vec();
        instr.reverse(); // going to be poppin' the instructions, so we'll just reverse them here
        instr
    }

    pub fn navigate(&mut self) {
        loop {
            match self.instructions.pop() {
                Some(i) => {
                    self.process_next_instruction(&i);
                    self.display_map();
                }
                None => {
                    println!("All done with the moves.");
                    break;
                }
            }
        }
    }

    fn process_next_instruction(&mut self, instruction: &str) {
        let mut chars = instruction.chars();
        let dir = match chars.next_back().unwrap() {
            'U' => Facing::UP,
            'R' => Facing::RIGHT,
            'D' => Facing::DOWN,
            'L' => Facing::LEFT,
            _ => unreachable!("invalid direction"),
        };
        let amount = chars.as_str().parse::<usize>().unwrap();
        println!("moving {amount} spaces in the {dir:?} direction");
        let mut steps_taken = 0;
        let mut next_x = self.pos.0;
        let mut next_y = self.pos.1;
        match dir {
            Facing::RIGHT => loop {
                next_x += 1;
                if next_x > self.max_x - 1 {
                    println!("at end of board, warping around");
                    next_x = 0;
                }
                if self.map.contains_key(&(next_x, next_y)) {
                    let contents = self.map.get(&(next_x, next_y)).unwrap();
                    match *contents {
                        Entity::WALL => {
                            println!("hit a wall, remaining where we are");
                            // steps_taken += 1;
                            break;
                        }
                        Entity::NONE => {
                            println!("we're in that blank space, need to take another step still");
                            // next_x += 1;
                        }
                        Entity::SPACE => {
                            println!("yay! somewhere to move");
                            self.pos = (next_x, next_y);
                            steps_taken += 1;
                        }
                        _ => {}
                    }
                } else {
                    println!("we're in no mans land after the last valid char in this row, just need to warp around, unless we can't");
                    // this is a special case, we have to watch out because if the next entity we encounter is a wall, then we can't actually update our pos
                    // self.pos = (next, self.pos.1.clone());
                    // next_x += 1;
                }
                if steps_taken == amount {
                    break;
                }
            },
            _ => {}
        };
    }

    pub fn display_map(&self) {
        for y in 0..self.max_x {
            for x in 0..self.max_x {
                if self.pos == (x, y) {
                    print!("{}", self.facing.0);
                } else {
                    let e = match self.map.get(&(x, y)) {
                        Some(entity) => entity.0,
                        None => Entity::NONE.0,
                    };
                    print!("{e}");
                }
            }
            println!("");
        }
    }
}

// naive first, we'll find the max length, then every space will be an Entity
// this means we'll have awkward movement checking, but hopefully easier than coming
// up with a decent data structure
pub fn parse(input: &str) -> (BTreeMap<(usize, usize), Entity>, usize, String) {
    let max = input.lines().fold(0, |acc, line| cmp::max(acc, line.len()));
    let m = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .map(move |(x, ch)| match ch {
                    '#' => Some(((x.clone(), y.clone()), Entity::WALL)),
                    '.' => Some(((x.clone(), y.clone()), Entity::SPACE)),
                    _ => None,
                })
        })
        .filter_map(|v| v)
        .collect();
    (m, max, String::from("10R5L5R10L4R5L5"))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (lookup, max_x, instructions) = parse(input);

    let mut player1 = Player::new(lookup, max_x, &instructions);
    player1.navigate();

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
