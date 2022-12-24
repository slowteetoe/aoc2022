use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct Knot {
    pos: (i16, i16),
    prev: (i16, i16),
    visited: BTreeSet<(i16, i16)>,
}

impl Knot {
    pub fn new() -> Self {
        let mut n = Self {
            pos: (0, 0),
            prev: (0, 0),
            visited: BTreeSet::new(),
        };
        n.visited.insert((0, 0));
        n
    }
}

#[derive(Debug)]
struct Board {
    knots: Vec<Knot>,
}

impl Board {
    pub fn new(num_knots: usize) -> Self {
        Board {
            knots: vec![Knot::new(); num_knots],
        }
    }

    pub fn process_move_cmd(&mut self, command: &str) {
        // println!("== {:?} ==", &command);
        let mut iter = command.split(' ');
        let dir = match iter.next().unwrap() {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!("or something went really wrong"),
        };
        let amount = iter.next().unwrap().parse::<i16>().unwrap();
        for _ in 0..amount {
            self.move_head(dir);
            for n in 1..self.knots.len() {
                self.move_n(n);
            }
            // self.show();
        }
    }

    pub fn move_head(&mut self, dir: (i16, i16)) {
        self.knots[0].prev = self.knots[0].pos;
        let curr = self.knots[0].pos;
        let next = (curr.0 + dir.0, curr.1 + dir.1);
        self.knots[0].pos = next;
        self.knots[0].visited.insert(next);
    }

    /// Move the nth knot to maintain the correct distance to the n-1 knot
    pub fn move_n(&mut self, n: usize) {
        let head = self.knots[n - 1].pos;
        let tail = self.knots[n].pos;

        let next_pos = if head.0 == tail.0 + 2 && head.1 == tail.1 {
            // move tail right one space
            (tail.0 + 1, tail.1)
        } else if head.0 == tail.0 - 2 && head.1 == tail.1 {
            // move tail left one space
            (tail.0 - 1, tail.1)
        } else if head.1 == tail.1 + 2 && head.0 == tail.0 {
            // move tail down one space
            (tail.0, tail.1 + 1)
        } else if head.1 == tail.1 - 2 && head.0 == tail.0 {
            // move tail up one space
            (tail.0, tail.1 - 1)
        } else {
            if Self::distance(head, tail) < 1.5 {
                (tail.0, tail.1)
            } else if head.0 > tail.0 && head.1 > tail.1 {
                (tail.0 + 1, tail.1 + 1)
            } else if head.0 < tail.0 && head.1 < tail.1 {
                (tail.0 - 1, tail.1 - 1)
            } else if head.0 < tail.0 && head.1 > tail.1 {
                (tail.0 - 1, tail.1 + 1)
            } else if head.0 > tail.0 && head.1 < tail.1 {
                (tail.0 + 1, tail.1 - 1)
            } else {
                (tail.0, tail.1)
            }
        };

        self.knots[n].prev = self.knots[n].pos;
        self.knots[n].pos = next_pos;
        self.knots[n].visited.insert(next_pos);
    }

    pub fn distance(head: (i16, i16), tail: (i16, i16)) -> f32 {
        let part1 = i32::pow((head.0 - tail.0) as i32, 2);
        let part2 = i32::pow((head.1 - tail.1) as i32, 2);
        f32::sqrt((part1 + part2) as f32)
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        let dim = self.knots.len() + 16; // we'll never stretch more than len() in any direction, but examples were 26 pixels
        let mut grid = vec![vec![String::from("."); dim]; dim];
        // center the display on the H and translate all coords
        let t = (
            self.knots[0].pos.0 * -1 + (dim as i16 / 2),
            self.knots[0].pos.1 * -1 + (dim as i16 / 2),
        );

        for n in (0..self.knots.len()).rev() {
            let virt = (self.knots[n].pos.0 + t.0, self.knots[n].pos.1 + t.1);

            let label = match n {
                0 => String::from("H"),
                _ => n.to_string(),
            };

            *grid
                .get_mut(virt.1 as usize)
                .expect("wrong row")
                .get_mut(virt.0 as usize)
                .expect("wrong col") = label;
        }

        for rows in grid.iter() {
            for cols in rows.iter() {
                print!("{}", cols);
            }
            println!();
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut b = Board::new(2);
    for cmd in input.lines() {
        b.process_move_cmd(cmd);
    }
    Some(b.knots[1].visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut b = Board::new(10);
    for cmd in input.lines() {
        b.process_move_cmd(cmd);
        // b.show();
    }
    Some(b.knots[9].visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_with_part("examples", 9, Some(2));
        assert_eq!(part_two(&input), Some(36));
    }
}
