use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::fmt::Display;

const OPEN: &str = " ";
const SAND: &str = "🐢";
const WALL: &str = "🧱";

#[derive(Debug)]
pub struct Point(usize, usize);

pub struct Grid {
    viewport: usize,
    data: Vec<Vec<String>>,
    virtual_floor: Option<usize>,
    virtual_floor_initialized: bool,
    highest: usize,
}

impl Grid {
    pub fn new(size: usize, add_virtual_floor: bool) -> Self {
        Self {
            viewport: size,
            data: vec![vec![String::from(OPEN); size]; size],
            virtual_floor: if add_virtual_floor { Some(0) } else { None },
            virtual_floor_initialized: false,
            highest: 0,
        }
    }

    pub fn draw_line(&mut self, start: &Point, end: &Point) {
        // println!("draw me a line from {:?} to {:?}", start, end);
        if start.0 == end.0 {
            for dy in cmp::min(start.1, end.1)..=(cmp::max(start.1, end.1)) {
                self.draw_point(start.0, dy);
            }
        } else {
            for dx in cmp::min(start.0, end.0)..=(cmp::max(start.0, end.0)) {
                self.draw_point(dx, start.1);
            }
        }
    }

    pub fn draw_point(&mut self, untranslated_x: usize, untranslated_y: usize) {
        self.data[untranslated_y][untranslated_x - (500 - (self.viewport / 2))] = WALL.to_string();
        self.highest = cmp::max(untranslated_y, self.highest);
    }

    pub fn drop_sand(&mut self) -> Option<Point> {
        if self.virtual_floor.is_some() && !self.virtual_floor_initialized {
            // it's too late to think this through, so I'll just create a physical 'virtual' wall!
            self.draw_line(
                &Point(500 - (self.viewport / 2), self.highest + 2),
                &Point(500 + (self.viewport / 2) - 1, self.highest + 2),
            );
            self.virtual_floor_initialized = true;
        }
        // inject a grain of sand at (500,0) and return the point it stops at.  Why do I need the point? I was going to do some cool animations/stepping
        let mut x = 500 - (500 - (self.viewport / 2));
        let mut y = 0;

        if self.data[y][x] == WALL || self.data[y][x] == SAND {
            println!("All jammed up");
            return None;
        }
        loop {
            if y + 1 == self.viewport {
                // dropping out of view
                return None;
            }
            if self.data[y + 1][x] == OPEN {
                // try to move down 1
                y += 1;
                continue;
            } else if self.data[y + 1][x - 1] == OPEN {
                // try to move down 1, left 1
                y += 1;
                x -= 1;
                continue;
            } else if self.data[y + 1][x + 1] == OPEN {
                // try to move down 1, right 1
                y += 1;
                x += 1;
                continue;
            }
            // println!("progress stopped, painting @ [{:?},{:?}]", x, y);
            self.data[y][x] = SAND.to_string();
            return Some(Point(x, y));
        }
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

pub fn parse_lines(input: &str) -> Vec<Vec<Point>> {
    let re = Regex::new(r"(\d+,\d+)+( -> )?").unwrap();
    let mut lines = vec![];
    for line in input.lines() {
        let pts = re
            .captures_iter(line)
            .filter_map(|c| Some(c.get(1).unwrap().as_str()))
            .map(|s| {
                let mut m = s.splitn(2, ",");
                Point(
                    m.next().unwrap().parse::<u16>().unwrap().into(),
                    m.next().unwrap().parse::<u16>().unwrap().into(),
                )
            })
            .collect_vec();
        lines.push(pts);
    }
    lines
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse_lines(input);
    let mut grid = Grid::new(200, false);
    for parts in lines {
        for (n, _) in parts.iter().enumerate() {
            if n == 0 {
                continue;
            }
            grid.draw_line(&parts[n - 1], &parts[n]);
        }
    }
    let mut grains = 0;
    while grid.drop_sand().is_some() {
        grains += 1;
        // uncomment to step through, <space> + enter, pretty fun to watch
        // println!("{}", grid);
        // let pause: String = read!(" {}");
    }
    println!("{}", grid);
    Some(grains)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = parse_lines(input);
    let mut grid = Grid::new(500, true);
    for parts in lines {
        for (n, _) in parts.iter().enumerate() {
            if n == 0 {
                continue;
            }
            grid.draw_line(&parts[n - 1], &parts[n]);
        }
    }
    let mut grains = 0;
    while grid.drop_sand().is_some() {
        grains += 1;
        // uncomment to step through, <space> + enter, pretty fun to watch
        // println!("{}", grid);
        // let pause: String = read!(" {}");
    }
    // println!("{}", grid);
    Some(grains)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
