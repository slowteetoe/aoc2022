use std::{
    cmp,
    collections::{BTreeMap, BTreeSet},
};

use advent_of_code::helpers::{intersection, manhattan};
use regex::Regex;

pub fn read_sensors(input: &str) -> (BTreeMap<(i64, i64), usize>, BTreeSet<(i64, i64)>) {
    // for part 1 at least, we don't really need to keep track of the beacon locations
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut sensor_map = BTreeMap::new();
    let mut beacons = BTreeSet::new();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let (x1, y1) = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
        );
        let (x2, y2) = (
            cap[3].parse::<i64>().unwrap(),
            cap[4].parse::<i64>().unwrap(),
        );
        sensor_map.insert((x1, y1), manhattan((x1, y1), (x2, y2)));
        beacons.insert((x2, y2));
    }

    // now that we have our sensor points with distances, and map bounds, let's create the map
    (sensor_map, beacons)
}

pub fn part_one(input: &str) -> Option<usize> {
    let sensors = read_sensors(input);

    // let target = 2_000_000i64;
    let target = 10i64;

    let mut ranges = vec![];

    for (sensor, dist) in sensors.0 {
        // now we're going to iterate over the sensor map and calculate the points where a beacon cannot exist
        // if that point intersects with y=200_000 then we'll add it to a set
        println!(
            "sensor({:?},{:?}) with scanning distance of {:?}",
            sensor.0, sensor.1, dist
        );
        if sensor.1.abs_diff(target) <= dist.try_into().unwrap() {
            // let's try dropping any work that can't hit our target
            println!("\tintersects target line");
            let d = dist as i64;
            let up = (sensor.0, sensor.1 + d);
            let right = (sensor.0 + d, sensor.1);
            let down = (sensor.0, sensor.1 - d);
            let left = (sensor.0 - d, sensor.1);

            let target_line = ((-1000, target), (1000, target));

            // now figure out which lines intersect target line, it should always be the \/ or /\ lines
            let p1 = intersection((left, down), target_line);
            if p1.is_some() {
                println!(
                    "intersected left down at {:?}, looking for other point on right down",
                    p1.unwrap()
                );
                let p2 = intersection((right, down), target_line);
                println!("intersected right down at {:?}", p2);
                ranges.push((p1.unwrap().0, p2.unwrap().0)); // these will all be on the y (target) axis
            } else {
                let p1 = intersection((left, up), target_line);
                if p1.is_none() {
                    panic!("I thought this intersected, but apparently not!");
                }
                let p2 = intersection((right, up), target_line);
                ranges.push((p1.unwrap().0, p2.unwrap().0));
            }
        } else {
            println!("\tcan't reach the target line");
            continue;
        }
    }

    // FIXME now we need to process the ranges, merging together
    ranges.sort();
    println!("sensor ranges (that we need to grow): {:?}", ranges);
    let final_ranges = collapse(ranges);
    if final_ranges.len() == 1 {
        return Some(final_ranges[0].1.abs_diff(final_ranges[0].0) as usize);
    }
    None
}

pub fn collapse(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    println!("**** collapsing: {:?}", ranges);
    if ranges.len() == 1 {
        println!("done, only one contiguous range left");
        return ranges;
    }
    if ranges.len() == 2 {
        println!("two ranges left, let's try to merge them");
        if ranges[0].1 > ranges[1].0 {
            println!("merging final pair");
            return vec![(ranges[0].0, cmp::max(ranges[0].1, ranges[1].1))];
        }

        return ranges;
    }

    let mut collapsed = vec![];

    // let's try to collapse every 2 ...
    for n in 1..=ranges.len() / 2 {
        let left = ranges[2 * n - 1 - 1];
        let right = ranges[2 * n - 1];
        println!("looking at {:?} and {:?}", left, right);
        if left.1 > right.0 {
            // merge
            let end = cmp::max(left.1, right.1);
            println!("merging ranges to {:?}", (left.0, end));
            collapsed.push((left.0, end));
        } else {
            println!("couldn't merge");
            collapsed.push(left);
            collapsed.push(right);
        }
    }
    if ranges.len() % 2 == 1 {
        println!("odd");
        // if there was an odd number, we didn't try to collapse the last val so add it back
        collapsed.push(ranges[ranges.len() - 1]);
    }

    return collapse(collapsed);
}

pub fn calculate_exclusion_zone(origin: (i64, i64), dist: usize, target_y: i64) {}

// https://www.reddit.com/r/adventofcode/comments/zmjzu7/2022_day_15_part_2_no_search_formula/
// -or- https://github.com/emlun/adventofcode-2022/blob/9dadc35bb4ebbb8352f9525f0eeaf2ea2babd766/src/days/day15.rs#L57-L91
pub fn part_two(input: &str) -> Option<u32> {
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
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
