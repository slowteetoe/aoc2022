use std::collections::BTreeSet;

#[derive(Debug)]
struct Board {
    visited: BTreeSet<(i16, i16)>,
    h_pos: (i16, i16),
    trailing: (i16, i16),
    t_pos: (i16, i16),
}

impl Board {
    pub fn new() -> Self {
        let mut b = Board {
            visited: BTreeSet::new(),
            h_pos: (0, 0),
            trailing: (0, 0),
            t_pos: (0, 0),
        };
        b.visited.insert(b.t_pos.clone());
        b
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
            self.move_tail();
        }
    }

    pub fn move_head(&mut self, dir: (i16, i16)) {
        self.trailing = self.h_pos;
        self.h_pos.0 += dir.0;
        self.h_pos.1 += dir.1;
    }

    pub fn move_tail(&mut self) {
        // if t_pos is within 1 space of h_pos, then just return
        let distance = Self::distance(self.h_pos, self.t_pos);
        // dbg!(&distance);
        if distance >= 2.0 {
            // println!(
            //     "moving tail {:?} to {:?}, a space toward head? {:?}, distance was {:?}",
            //     self.t_pos, self.trailing, self.h_pos, distance
            // );
            self.t_pos = self.trailing;
            self.visited.insert(self.t_pos);
        } else {
            // println!("*** skipping tail move, within 1 space ***");
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
    let mut b = Board::new();
    for cmd in input.lines() {
        b.process_move_cmd(cmd);
    }
    Some(b.visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
