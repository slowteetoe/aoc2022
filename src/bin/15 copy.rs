use std::collections::{BTreeMap, BTreeSet};

use regex::Regex;

pub fn read_sensors(input: &str) -> (BTreeMap<(i32, i32), usize>, BTreeSet<(i32, i32)>) {
    // for part 1 at least, we don't really need to keep track of the beacon locations
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut sensor_map = BTreeMap::new();
    let mut beacons = BTreeSet::new();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let (x1, y1) = (
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        let (x2, y2) = (
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );
        sensor_map.insert((x1, y1), manhattan((x1, y1), (x2, y2)));
        beacons.insert((x2, y2));
    }

    // now that we have our sensor points with distances, and map bounds, let's create the map
    (sensor_map, beacons)
}

pub fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> usize {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let sensors = read_sensors(input);
    let mut excluded = BTreeSet::new();

    // let target = 2_000_000i32;
    let target = 10i32;

    for (sensor, dist) in sensors.0 {
        // now we're going to iterate over the sensor map and calculate the points where a beacon cannot exist
        // if that point intersects with y=200_000 then we'll add it to a set

        if sensor.1.abs_diff(target) > dist.try_into().unwrap() {
            // let's try dropping any work that can't hit our target
            println!("skipping ({:?},{:?})", sensor.0, sensor.1);
            continue;
        }

        calculate_exclusion_zone(sensor, dist, target)
            .iter()
            .filter(|(_x, y)| *y == target)
            .for_each(|pt| {
                excluded.insert(pt.clone());
            });
    }
    for pt in sensors.1 {
        if pt.1 == target {
            println!("Beacon at {:?}", &pt);
            excluded.remove(&pt.clone());
        }
    }
    Some(excluded.len())
}

pub fn calculate_exclusion_zone(
    origin: (i32, i32),
    dist: usize,
    target_y: i32,
) -> BTreeSet<(i32, i32)> {
    // I'm tired, lets just be inefficient - OOPS this only works for the examples, guess I have to come up with an actual algorithm
    let mut candidates = BTreeSet::new();
    candidates.insert((origin.0, origin.1));

    for x in (origin.0 - dist as i32)..=(origin.0 + dist as i32) {
        for y in (origin.1 - dist as i32)..=(origin.1 + dist as i32) {
            if y == target_y {
                candidates.insert((x, y));
            }
        }
    }

    // println!(
    //     "candidates within distance {} of {:?} are: {:?}",
    //     dist, &origin, candidates,
    // );
    candidates
        .into_iter()
        .filter(|pt| manhattan(origin, *pt) <= dist)
        .collect()
}

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

    // #[test]
    // fn test_exclusion_zone() {
    //     let result = calculate_exclusion_zone((0, 0), 1);
    //     assert_eq!(
    //         result.len(),
    //         vec![(0, 1), (1, 0), (-1, 0), (0, -1), (0, 0)].len()
    //     );
    //     let result = calculate_exclusion_zone((0, 0), 2);
    //     assert_eq!(result.len(), 13);

    //     let result = calculate_exclusion_zone((0, 0), 3);
    //     assert_eq!(result.len(), 25);

    //     let result = calculate_exclusion_zone((0, 0), 4);
    //     assert_eq!(result.len(), 41);
    // }
}
