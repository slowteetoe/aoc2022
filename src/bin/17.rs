use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

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

#[derive(Debug)]
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
            .par_iter()
            .any(|(x, y)| self.data.get(&(desired.0 + x, desired.1 - y)).is_some())
    }
}

pub fn play_sorta_tetris(shapes: Vec<Shape>, moves: Vec<Move>, limit: u128) -> Option<u128> {
    // we'll just keep repeating the same moves and shapes
    let mut moves = moves.iter().cycle();
    let mut shapes = shapes.iter().cycle();

    let mut grid = Grid {
        data: BTreeMap::new(),
    };

    // insert a floor to make easier
    for x in 0..7 {
        grid.data.insert((x, 0), Rock::Rock);
    }

    let mut rocks_stopped = 0;

    while rocks_stopped != limit {
        if rocks_stopped % 100_000 == 0 {
            println!("{rocks_stopped}");
        }
        let this_shape = shapes.next().unwrap();
        let start_pos = *grid.data.keys().map(|(_, y)| y).max().unwrap_or(&0);
        let mut curr_pos: (u128, u128) = (2, start_pos + 3 + this_shape.height());

        loop {
            let current_position = match moves.next().unwrap() {
                // we can move left all the way to 0 since we're keeping track of shape at 0,0 - but we have to account for the width of the shape when moving right
                Move::Left => {
                    if let Some(x_pos) = curr_pos.0.checked_sub(1) {
                        let desired = (x_pos, curr_pos.1);
                        if grid.can_place_shape(this_shape, desired) {
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
                    if curr_pos.0 == 7 - this_shape.max_width()
                        || !grid.can_place_shape(this_shape, desired)
                    {
                        curr_pos
                    } else {
                        desired
                    }
                }
            };
            // try to drop it down
            let desired_next_position = (current_position.0, current_position.1 - 1);
            if grid.can_place_shape(this_shape, desired_next_position) {
                // set next position
                curr_pos = desired_next_position;
            } else {
                // stick shape at current pos
                for position in this_shape.offsets.iter() {
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
    }

    Some(*grid.data.keys().map(|(_, y)| y).max().unwrap())
}

pub fn part_one(input: &str) -> Option<u128> {
    let (_, shapes) = shapes(ROCKS).unwrap();
    let (_, moves) = moves(input).unwrap();

    play_sorta_tetris(shapes, moves, 2022)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (_, shapes) = shapes(ROCKS).unwrap();
    let (_, moves) = moves(input).unwrap();

    // play_sorta_tetris(shapes, moves, 1_000_000_000_000)
    None
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
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
