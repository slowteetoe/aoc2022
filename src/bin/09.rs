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
        // println!("** {:?} **", &command);
        let mut iter = command.split(' ');
        let dir = match iter.next().unwrap() {
            "U" => (0, 1),
            "D" => (0, -1),
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
        let distance = Self::distance(self.knots[n - 1].pos, self.knots[n].pos);
        if distance >= 2.0 {
            let trailing = self.knots[n - 1].prev;
            self.knots[n].pos = trailing;
            self.knots[n].visited.insert(trailing);
        }
    }

    pub fn distance(head: (i16, i16), tail: (i16, i16)) -> f32 {
        // guess we'll have to figure this out, manhattan doesn't work for our diagonal
        // (head.0 - tail.0).abs() + (head.1 - tail.1).abs()
        // plain distance?
        let part1 = i32::pow((head.0 - tail.0) as i32, 2);
        let part2 = i32::pow((head.1 - tail.1) as i32, 2);

        f32::sqrt((part1 + part2) as f32)
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
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
