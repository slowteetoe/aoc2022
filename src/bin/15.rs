use std::cell::RefCell;

use advent_of_code::helpers;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{self, line_ending};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

// ick, this is the first time the AoC macros have bitten me, the example and actual problem use different
// values that are not read from the input
thread_local!(static TARGET_ROW: RefCell<i32> = RefCell::new(2_000_000));
thread_local!(static MAX_DIM: RefCell<i32> = RefCell::new(4_000_000));

#[derive(Debug)]
pub struct Sensor {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Beacon {
    x: i32,
    y: i32,
}

// nom nom nom nom...
pub fn location(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

pub fn sensor_location(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, (x, y)) = location(input)?;
    Ok((input, Sensor { x, y }))
}

pub fn beacon_location(input: &str) -> IResult<&str, Beacon> {
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, (x, y)) = location(input)?;
    Ok((input, Beacon { x, y }))
}

pub fn sensor_beacon_pair(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, (sensor, beacon)) =
        terminated(tuple((sensor_location, beacon_location)), opt(line_ending))(input)?;
    Ok((input, (sensor, beacon)))
}

pub fn parse(input: &str) -> IResult<&str, Vec<(Sensor, Beacon)>> {
    many1(sensor_beacon_pair)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, locations) = parse(input).unwrap();
    // println!("{:?}", locations);

    let mut target_y: i32 = 0;
    TARGET_ROW.with(|target_row| {
        target_y = *target_row.borrow();
    });

    // which sensors have a range that overlaps the target row?
    let distances = locations
        .iter()
        .map(|(s, b)| {
            (
                s,
                helpers::manhattan((s.x as i64, s.y as i64), (b.x as i64, b.y as i64)),
            )
        })
        .filter_map(|(s, dist)| {
            if ((s.y - dist as i32)..(s.y + dist as i32)).contains(&(target_y as i32)) {
                Some((s, dist))
            } else {
                None
            }
        })
        .collect::<Vec<(&Sensor, usize)>>();

    // println!("Sensors that reach target row: {:?}", distances);

    // how many spaces are projected onto that row?
    let x_pos = distances
        .iter()
        .map(|(sensor, dist)| {
            let distance_to_line = sensor.y - target_y;
            let distance_on_line = *dist as i32 - distance_to_line.abs();
            sensor.x - distance_on_line..sensor.x + distance_on_line
        })
        .flatten()
        .unique()
        .collect_vec();

    Some(x_pos.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, locations) = parse(input).unwrap();

    let mut max_dim: i32 = 0;
    MAX_DIM.with(|dim| {
        max_dim = *dim.borrow();
    });
    println!("target is: (0,0) to ({:?},{:?})", max_dim, max_dim);

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        TARGET_ROW.with(|target_row| {
            *target_row.borrow_mut() = 10;
        });
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        MAX_DIM.with(|dim| {
            *dim.borrow_mut() = 20;
        });
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
