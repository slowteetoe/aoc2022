use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};
const GRID_HASH_LENGTH: u128 = 300;

// TETRIS!
const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rock {
    Rock,
    Space,
}

#[derive(Debug)]
pub struct Shape {
    rocks: Vec<Vec<Rock>>,
    offsets: Vec<(u128, u128)>,
}

impl Shape {
    pub fn new(offsets: Vec<(u128, u128)>, rocks: Vec<Vec<Rock>>) -> Self {
        Self { rocks, offsets }
    }
    pub fn height(&self) -> u128 {
        self.rocks.len() as u128
    }
    pub fn max_width(&self) -> u128 {
        self.rocks
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|rock| match rock {
                        Rock::Rock => true,
                        Rock::Space => false,
                    })
                    .count()
            })
            .max()
            .unwrap() as u128
    }
}

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
}

pub fn shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1(
        tag("\n\n"),
        separated_list1(
            line_ending,
            many1(alt((
                complete::char('#').map(|_| Rock::Rock),
                complete::char('.').map(|_| Rock::Space),
            ))),
        )
        .map(|rocks| {
            Shape::new(
                rocks
                    .iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        row.iter().enumerate().filter_map(move |(x, r)| match r {
                            Rock::Rock => Some((x as u128, y as u128)),
                            Rock::Space => None,
                        })
                    })
                    .collect::<Vec<(u128, u128)>>(),
                rocks,
            )
        }),
    )(input)
}

pub fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        complete::char('<').map(|_| Move::Left),
        complete::char('>').map(|_| Move::Right),
    )))(input)
}

pub struct Grid {
    data: BTreeMap<(u128, u128), Rock>,
}

impl Grid {
    fn can_place_shape(&self, this_shape: &Shape, desired: (u128, u128)) -> bool {
        !this_shape
            .offsets
            .iter()
            .any(|(x, y)| self.data.get(&(desired.0 + x, desired.1 - y)).is_some())
    }
    fn max_y(&self) -> u128 {
        *self.data.keys().map(|(_, y)| y).max().unwrap_or(&0)
    }
    fn snapshot(&self) -> Vec<u8> {
        let mut snap = vec![];
        // ugh
        for y in 0..GRID_HASH_LENGTH {
            for x in 0..7 {
                match self.data.get(&(x, y)) {
                    Some(r) => {
                        if *r == Rock::Rock {
                            snap.push(1);
                        } else {
                            snap.push(0);
                        }
                    }
                    None => {
                        snap.push(0);
                    }
                };
            }
        }
        snap
    }
}

// modifying the response to indicate if we found a cycle or not (repeat, height)
pub fn play_sorta_tetris(
    shapes: Vec<Shape>,
    moves: Vec<Move>,
    limit: u128,
    detect_cycle_only: bool,
) -> (Option<u128>, Option<(u128, u128)>) {
    // we'll just keep repeating the same moves and shapes
    let mut moves = moves.iter().enumerate().cycle();
    let mut shapes = shapes.iter().enumerate().cycle();

    let mut grid = Grid {
        data: BTreeMap::new(),
    };

    // insert a floor to make easier
    for x in 0..7 {
        grid.data.insert((x, 0), Rock::Rock);
    }

    let mut rocks_stopped = 0;

    // (last n lines of grid maybe, shape index, move index)
    let mut cache: BTreeMap<(Vec<u8>, usize, usize), Vec<(u128, u128)>> = BTreeMap::new();

    while rocks_stopped != limit {
        let this_shape = shapes.next().unwrap();
        let start_pos = &grid.max_y();
        let mut curr_pos: (u128, u128) = (2, start_pos + 3 + this_shape.1.height());

        let mut last_move_idx;
        loop {
            let this_move = moves.next().unwrap();
            last_move_idx = this_move.0;
            let current_position = match this_move.1 {
                // we can move left all the way to 0 since we're keeping track of shape at 0,0 - but we have to account for the width of the shape when moving right
                Move::Left => {
                    if let Some(x_pos) = curr_pos.0.checked_sub(1) {
                        let desired = (x_pos, curr_pos.1);
                        if grid.can_place_shape(this_shape.1, desired) {
                            desired
                        } else {
                            curr_pos
                        }
                    } else {
                        curr_pos
                    }
                }
                Move::Right => {
                    let desired = (curr_pos.0 + 1, curr_pos.1);
                    if curr_pos.0 == 7 - this_shape.1.max_width()
                        || !grid.can_place_shape(this_shape.1, desired)
                    {
                        curr_pos
                    } else {
                        desired
                    }
                }
            };
            // try to drop it down
            let desired_next_position = (current_position.0, current_position.1 - 1);
            if grid.can_place_shape(this_shape.1, desired_next_position) {
                // set next position
                curr_pos = desired_next_position;
            } else {
                // stick shape at current pos
                for position in this_shape.1.offsets.iter() {
                    grid.data.insert(
                        (
                            position.0 + current_position.0,
                            current_position.1 - position.1,
                        ),
                        Rock::Rock,
                    );
                }
                rocks_stopped += 1;
                break;
            }
        }
        // we successfully placed a block, we're looking back n lines of grid
        // (last n lines of grid, shape index, move index) => (cycle_length, grid_height)
        if curr_pos.1 > GRID_HASH_LENGTH && detect_cycle_only {
            // how do we build up something meaningful for the state of the grid?
            let snap = grid.snapshot();
            let key = (snap, this_shape.0, last_move_idx);
            let entry = cache
                .entry(key.clone())
                .and_modify(|v| v.push((rocks_stopped, grid.max_y())))
                .or_insert(vec![(rocks_stopped, grid.max_y())]);
            if entry.len() > 2 {
                // println!("We've seen this state before! {:?} => {:?}", key, entry);
                let second = entry.get(1).unwrap();
                let first = entry.get(0).unwrap();
                let cycle_length = second.0 - first.0;
                let tower_height = second.1 - first.1;
                return (None, Some((cycle_length, tower_height)));
            }
        }
    }

    (Some(*grid.data.keys().map(|(_, y)| y).max().unwrap()), None)
}

pub fn part_one(input: &str) -> Option<u128> {
    let (_, shapes1) = shapes(ROCKS).unwrap();
    let (_, moves1) = moves(input).unwrap();

    let limit = 2022;

    // by making the cycle detection keep track of enough of the board to detect the cycle in part 2, we made it require too much data for part one so it fails to detect a cycle
    // luckily, just letting it play out takes <50ms in release mode for part 1
    let (tower_height, _) = play_sorta_tetris(shapes1, moves1, limit, false);
    tower_height
}

pub fn part_two(input: &str) -> Option<u128> {
    let (_, shapes1) = shapes(ROCKS).unwrap();
    let (_, moves1) = moves(input).unwrap();

    let limit = 1_000_000_000_000;

    // first, detect the cycle
    let (_, cycle) = play_sorta_tetris(shapes1, moves1, limit, true);
    let (cycle_length, cycle_height) = cycle.unwrap();

    let complete_cycles = limit / cycle_length;
    let incomplete_cycles = limit % cycle_length;

    println!("Detected a cycle, every {cycle_length} shapes the tower will grow {cycle_height}.  This is {complete_cycles} complete cycles, with {incomplete_cycles} to play out...");

    // then, play out the remaining rounds
    // meh just reread rather than fix the borrowing
    let (_, shapes) = shapes(ROCKS).unwrap();
    let (_, moves) = moves(input).unwrap();

    let (played_height, _) = play_sorta_tetris(shapes, moves, incomplete_cycles, false);

    Some(played_height.unwrap() + cycle_height * complete_cycles)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
