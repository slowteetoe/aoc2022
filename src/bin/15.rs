use std::borrow::Borrow;
use std::cell::RefCell;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{terminated, tuple};
use nom::IResult;

// ick, this is the first time the AoC macros have really bitten me, the example and actual problem use different
// values that are not read from the input
thread_local!(static GLOBAL_DATA: RefCell<isize> = RefCell::new(10));

// is this better than just Regex captures? Dunno. But it's a chance to play with Nom
pub fn sensor_location(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, x) = nom::character::complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = nom::character::complete::i32(input)?;
    Ok((input, (x as isize, y as isize)))
}

pub fn beacon_location(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, x) = nom::character::complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = nom::character::complete::i32(input)?;
    Ok((input, (x as isize, y as isize)))
}

pub fn line(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
    let (input, (beacon, sensor)) =
        terminated(tuple((sensor_location, beacon_location)), opt(line_ending))(input)?;
    Ok((input, (beacon, sensor)))
}

pub fn parse(input: &str) -> IResult<&str, Vec<((isize, isize), (isize, isize))>> {
    many1(line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    // vec<(beacons, sensors)>
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
