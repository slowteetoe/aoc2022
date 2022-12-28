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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    max_dim: (usize, usize),                // helpful for display
    path: BTreeMap<(usize, usize), Facing>, // ugh, need to debug - would have been MUCH easier to use a vector
}
impl Player {
    pub fn new(
        map: BTreeMap<(usize, usize), Entity>,
        max_dim: (usize, usize),
        instructions: &str,
    ) -> Self {
        // initial pos = leftmost cell of top row in map
        let x_pos = map
            .iter()
            .map(|(k, v)| match (k.1, v) {
                (0, &Entity::SPACE) => k.0,
                _ => usize::MAX,
            })
            .min()
            .unwrap();
        let facing = Facing::RIGHT;
        println!("Initial pos: ({x_pos}, 0)");
        let mut path = BTreeMap::new();
        path.insert((x_pos, 0), facing.clone());
        Self {
            pos: ({ x_pos }, 0),
            facing,
            instructions: Self::parse_instructions(instructions),
            map,
            max_dim,
            path,
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
                    // self.display_map();
                }
                None => {
                    println!("All done with the moves.");
                    break;
                }
            }
        }
    }

    fn process_next_instruction(&mut self, instruction: &str) {
        println!("** * {instruction} * **");
        let mut chars = instruction.chars();
        // yes, probably a ringbuffer or even vector would be cleaner
        // you don't update facing until after moving the amount of spaces
        let dir = match chars.next_back().unwrap() {
            'R' => match self.facing {
                Facing::RIGHT => Facing::DOWN,
                Facing::DOWN => Facing::LEFT,
                Facing::LEFT => Facing::UP,
                Facing::UP => Facing::RIGHT,
                _ => unreachable!(),
            },
            'L' => match self.facing {
                Facing::RIGHT => Facing::UP,
                Facing::DOWN => Facing::RIGHT,
                Facing::LEFT => Facing::DOWN,
                Facing::UP => Facing::LEFT,
                _ => unreachable!(),
            },
            _ => unreachable!("invalid direction"),
        };
        let amount = chars.as_str().parse::<usize>().unwrap();
        println!("moving {amount} spaces in the {:?} direction", self.facing);
        let mut steps_taken = 0;
        let mut next_x = self.pos.0;
        let mut next_y = self.pos.1;

        let right_check = {
            |next_x: usize| {
                if next_x > self.max_dim.0 {
                    0
                } else {
                    next_x + 1
                }
            }
        };

        let left_check = {
            |next_x| {
                if next_x == 0 {
                    self.max_dim.0
                } else {
                    next_x - 1
                }
            }
        };

        let up_check = {
            |next_y| {
                if next_y == 0 {
                    self.max_dim.1
                } else {
                    next_y - 1
                }
            }
        };

        let down_check = {
            |next_y| {
                if next_y > self.max_dim.1 {
                    0
                } else {
                    next_y + 1
                }
            }
        };

        loop {
            match self.facing {
                Facing::RIGHT => {
                    next_x = right_check(next_x);
                }
                Facing::DOWN => {
                    next_y = down_check(next_y);
                }
                Facing::LEFT => {
                    next_x = left_check(next_x);
                }
                Facing::UP => {
                    next_y = up_check(next_y);
                }
                _ => unreachable!("no other directions"),
            };

            if self.map.contains_key(&(next_x, next_y)) {
                let contents = self.map.get(&(next_x, next_y)).unwrap();
                match *contents {
                    Entity::WALL => {
                        println!(
                            "hit wall traveling {} after {steps_taken} steps, stuck at {:?}",
                            self.facing.0, self.pos
                        );
                        break;
                    }
                    Entity::NONE => {}
                    Entity::SPACE => {
                        self.pos = (next_x, next_y);
                        self.path.insert((next_x, next_y), self.facing.clone());
                        steps_taken += 1;
                    }
                    _ => {}
                }
            }
            if steps_taken == amount {
                println!(
                    "successfully walked {} {steps_taken} steps, at {:?}",
                    self.facing.0, self.pos
                );
                break;
            }
        }

        // NOW update the direction based on the instruction
        self.facing = dir.clone();
    }

    pub fn display_map(&self) {
        for y in 0..self.max_dim.1 {
            for x in 0..self.max_dim.0 {
                if self.pos == (x, y) {
                    print!("{}", self.facing.0);
                } else {
                    let mut e = match self.map.get(&(x, y)) {
                        Some(entity) => entity.0,
                        None => Entity::NONE.0,
                    };

                    if self.path.contains_key(&(x, y)) {
                        // overlay our path to date
                        e = self.path.get(&(x, y)).unwrap().0;
                    }
                    print!("{e}");
                }
            }
            println!("");
        }
    }

    pub fn score(&self) -> usize {
        // looks like it's one-based
        let final_pos = (self.pos.0 + 1, self.pos.1 + 1);
        println!("final pos {:?}  and facing {:?}", final_pos, self.facing);
        (1000 * final_pos.1) + (4 * final_pos.0) + self.facing.1
    }
}

// naive first, we'll find the max length, then every space will be an Entity
// this means we'll have awkward movement checking, but hopefully easier than coming
// up with a decent data structure
pub fn parse(input: &str) -> (BTreeMap<(usize, usize), Entity>, (usize, usize), String) {
    let max_x = input
        .lines()
        .filter(|line| line.chars().all(|c| !c.is_alphanumeric()))
        .fold(0, |acc, line| cmp::max(acc, line.len()));
    let max_y = input.lines().count() - 2; // subtract out newline and instructions
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
    (m, (max_x, max_y), input.lines().last().unwrap().to_string())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (lookup, max_dim, instructions) = parse(input);

    let mut p = Player::new(lookup, max_dim, &instructions);
    p.navigate();
    // 197160 is correct, should end up at (39, 196)
    Some(p.score())
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
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
