use std::{collections::BTreeMap, fmt::Display};

use regex::Regex;
use std::cmp;

const OPEN: &str = ".";
const SENSOR: &str = "S";
const EXCLUDED: &str = "#";

pub struct Grid {
    // maybe?
    min_y: i32,
    max_y: i32,
    min_x: i32,
    max_x: i32,

    data: Vec<Vec<String>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        // for part 1 at least, we don't really need to keep track of the beacon locations
        let mut grid = Self {
            min_y: 0,
            max_y: 0,
            min_x: 0,
            max_x: 0,
            data: vec![vec![]],
        };

        let mut sensor_map = BTreeMap::new();
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

            grid.min_x = cmp::min(grid.min_x, cmp::min(x1, x2));
            grid.max_x = cmp::max(grid.max_x, cmp::max(x1, x2));

            grid.min_y = cmp::min(grid.min_y, cmp::min(y1, y2));
            grid.max_y = cmp::max(grid.max_y, cmp::max(y1, y2));
        }

        // now that we have our sensor points with distances, and map bounds, let's create the map
        dbg!(&sensor_map);

        let dy = grid.max_y - grid.min_y;
        let dx = grid.max_x - grid.min_x;
        let dim = cmp::max(dx, dy) as usize;
        println!("building a {}x{} map", dim, dim);
        let delta = 2usize; // FIXME have to figure out how to compute this

        let mut data = vec![vec![OPEN.to_string(); dim]; dim];

        for (s, dist) in &sensor_map {
            // hmm... need to translate to our virtual coords
            let sx = s.0 as usize + delta;
            let sy = s.1 as usize + delta;

            data[sy][sx] = SENSOR.to_string();
            // now paint outward, there's definitely a better way!
            println!("Trying to paint to sides of ({},{})", sx, sy);
            for d in 0..*dist {
                if sy - d > data.len() - 1 || sy - d <= 0 {
                    // ignore beyond the boundaries of the map
                    continue;
                }
                // straight up, plus branch one more level each time
                data[sy - d][sx] = EXCLUDED.to_string();
                for width in 0..(*dist - d) {
                    if (sx - width) >= 0 {
                        data[sy - d][sx - width] = EXCLUDED.to_string();
                    }
                    if sx + width < dim - 1 {
                        data[sy - d][sx + width] = EXCLUDED.to_string();
                    }
                }
            }
        }
        grid.data = data;
        grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                f.write_str(&col)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> usize {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs())
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);
    println!("{}", grid);
    None
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
}
