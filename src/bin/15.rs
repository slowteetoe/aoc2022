use std::cell::RefCell;

use nom::bytes::complete::tag;
use nom::character::complete::{self, line_ending};
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

// ick, this is the first time the AoC macros have bitten me, the example and actual problem use different
// values that are not read from the input
thread_local!(static GLOBAL_DATA: RefCell<isize> = RefCell::new(10));

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

pub fn part_one(input: &str) -> Option<u32> {
    let (_, locations) = parse(input).unwrap();
    println!("{:?}", locations);

    GLOBAL_DATA.with(|target_row| {
        println!("target is: {:?}", *target_row.borrow());
    });
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    GLOBAL_DATA.with(|target_row| {
        *target_row.borrow_mut() = 2_000_000;
    });
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
