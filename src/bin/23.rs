use core::num;
use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashMap},
};

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i32, i32);

// I don't think I want a grid for part one, let's just work with points
pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, ch)| {
                if ch == '#' {
                    Some(Point(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect_vec()
}

pub fn vectors() -> BTreeMap<Point, Vec<Point>> {
    let mut directions = BTreeMap::new();
    directions.insert(
        // North
        Point(0, -1),
        vec![Point(-1, -1), Point(0, -1), Point(1, -1)],
    );
    directions.insert(
        // South
        Point(0, 1),
        vec![Point(-1, 1), Point(0, 1), Point(1, 1)],
    );
    directions.insert(
        // East
        Point(1, 0),
        vec![Point(1, -1), Point(1, 0), Point(1, 1)],
    );
    directions.insert(
        // West
        Point(-1, 0),
        vec![Point(-1, -1), Point(-1, 0), Point(-1, 1)],
    );

    directions
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = parse(input);
    // dbg!(&elves);
    let mut dir_priority = vec![Point(0, -1), Point(0, 1), Point(-1, 0), Point(1, 0)];
    let vectors = vectors();

    for round in 1..=10 {
        let mut proposals = vec![]; // elf at Point is going to want to move to Point
        'nextelf: for elf in elves.iter() {
            // println!("** ELF @ {:?}", &elf);
            // ugh, missed that we have to check each of the 8 surrounding spaces to make sure there's at least one elf
            // probably should have gone with a bit vec or something more efficient instead of doing the same checks many times
            if !(elves.contains(&Point(elf.0 - 1, elf.1 - 1))
                || elves.contains(&Point(elf.0, elf.1 - 1))
                || elves.contains(&Point(elf.0 + 1, elf.1 - 1))
                || elves.contains(&Point(elf.0 - 1, elf.1))
                || elves.contains(&Point(elf.0 + 1, elf.1))
                || elves.contains(&Point(elf.0 - 1, elf.1 + 1))
                || elves.contains(&Point(elf.0, elf.1 + 1))
                || elves.contains(&Point(elf.0 + 1, elf.1 + 1)))
            {
                // println!("no nearby elves, not going anywhere");
                proposals.push((elf.clone(), None));
                continue;
            }
            // check in order for the different directions
            'dircheck: for cardinal in dir_priority.iter() {
                // println!("{:?} is checking cardinal direction {:?}", &elf, &cardinal);
                // grab the different vectors and see if there's anyone interfering
                let v = vectors.get(&cardinal).unwrap();
                for Point(dx, dy) in v {
                    let target = Point(elf.0 + dx, elf.1 + dy);
                    if elves.contains(&target) {
                        // println!(
                        //     "elf in the way @ {:?}! checking a different direction",
                        //     target
                        // );
                        continue 'dircheck;
                    }
                }
                let proposal = Point(elf.0 + cardinal.0, elf.1 + cardinal.1);
                // println!("elf at {:?} proposes moving to {:?}", &elf, &proposal);
                proposals.push((elf.clone(), Some(proposal)));
                continue 'nextelf;
            }
            // println!("elf couldn't find somewhere to go, staying put!");
            proposals.push((elf.clone(), None));
        }

        // dbg!(&proposals);
        elves.clear();
        // resolve the proposals (and who stayed in place)
        let mut did_not_move = proposals
            .iter()
            .filter_map(|(elf, prop)| {
                if prop.is_none() {
                    Some(elf.clone())
                } else {
                    None
                }
            })
            .collect_vec()
            .to_vec();
        elves.append(&mut did_not_move);

        // probably would have been more clear to use did_not_move, but it's the same thing since we cleared the elves
        let mut collisions: HashMap<Point, u8> = elves.iter().map(|elf| (elf.clone(), 1)).collect();

        proposals
            .iter()
            .filter(|(_, target)| target.is_some())
            .for_each(|(_, target)| {
                collisions
                    .entry(target.as_ref().unwrap().clone())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            });

        // dbg!(&collisions);

        proposals
            .iter()
            .filter(|(_, target)| target.is_some())
            .for_each(|(elf, target)| {
                if collisions.get(target.as_ref().unwrap()).unwrap() > &1 {
                    // println!(
                    //     "elf couldn't move to their proposed location, remaining at {:?}",
                    //     &elf
                    // );
                    elves.push(elf.clone());
                } else {
                    // println!("yay, elf can move to their proposed location");
                    elves.push(target.as_ref().unwrap().clone());
                }
            });

        // println!("new state\n\n{:?}\n\n", &elves);
        dir_priority.rotate_left(1);
        // println!("New dir priority = {:?}", dir_priority);
    }
    // find furthest N E S W elves, this makes up the final grid dimensions to calculate
    //
    let (min_x, max_x, min_y, max_y) =
        elves
            .iter()
            .fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |acc, p| {
                (
                    min(acc.0, p.0),
                    max(acc.1, p.0),
                    min(acc.2, p.1),
                    max(acc.3, p.1),
                )
            });
    let spaces = ((max_x - min_x) + 1) * ((max_y - min_y) + 1); // yeah, account for 0-based
    Some(spaces as u32 - elves.len() as u32)
}

// doesn't get much hackier than C+P!
pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = parse(input);
    let mut dir_priority = vec![Point(0, -1), Point(0, 1), Point(-1, 0), Point(1, 0)];
    let vectors = vectors();

    let num_elves = elves.len();

    for round in 1..=u32::MAX {
        let mut moved = 0;
        let mut proposals = vec![]; // elf at Point is going to want to move to Point
        'nextelf: for elf in elves.iter() {
            // println!("** ELF @ {:?}", &elf);
            // ugh, missed that we have to check each of the 8 surrounding spaces to make sure there's at least one elf
            // probably should have gone with a bit vec or something more efficient instead of doing the same checks many times
            if !(elves.contains(&Point(elf.0 - 1, elf.1 - 1))
                || elves.contains(&Point(elf.0, elf.1 - 1))
                || elves.contains(&Point(elf.0 + 1, elf.1 - 1))
                || elves.contains(&Point(elf.0 - 1, elf.1))
                || elves.contains(&Point(elf.0 + 1, elf.1))
                || elves.contains(&Point(elf.0 - 1, elf.1 + 1))
                || elves.contains(&Point(elf.0, elf.1 + 1))
                || elves.contains(&Point(elf.0 + 1, elf.1 + 1)))
            {
                // println!("no nearby elves, not going anywhere");
                proposals.push((elf.clone(), None));
                continue;
            }
            // check in order for the different directions
            'dircheck: for cardinal in dir_priority.iter() {
                // println!("{:?} is checking cardinal direction {:?}", &elf, &cardinal);
                // grab the different vectors and see if there's anyone interfering
                let v = vectors.get(&cardinal).unwrap();
                for Point(dx, dy) in v {
                    let target = Point(elf.0 + dx, elf.1 + dy);
                    if elves.contains(&target) {
                        // println!(
                        //     "elf in the way @ {:?}! checking a different direction",
                        //     target
                        // );
                        continue 'dircheck;
                    }
                }
                let proposal = Point(elf.0 + cardinal.0, elf.1 + cardinal.1);
                // println!("elf at {:?} proposes moving to {:?}", &elf, &proposal);
                proposals.push((elf.clone(), Some(proposal)));
                continue 'nextelf;
            }
            // println!("elf couldn't find somewhere to go, staying put!");
            proposals.push((elf.clone(), None));
        }

        // dbg!(&proposals);
        elves.clear();
        // resolve the proposals (and who stayed in place)
        let mut did_not_move = proposals
            .iter()
            .filter_map(|(elf, prop)| {
                if prop.is_none() {
                    Some(elf.clone())
                } else {
                    None
                }
            })
            .collect_vec()
            .to_vec();
        elves.append(&mut did_not_move);

        // probably would have been more clear to use did_not_move, but it's the same thing since we cleared the elves
        let mut collisions: HashMap<Point, u8> = elves.iter().map(|elf| (elf.clone(), 1)).collect();

        proposals
            .iter()
            .filter(|(_, target)| target.is_some())
            .for_each(|(_, target)| {
                collisions
                    .entry(target.as_ref().unwrap().clone())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            });

        // dbg!(&collisions);

        proposals
            .iter()
            .filter(|(_, target)| target.is_some())
            .for_each(|(elf, target)| {
                if collisions.get(target.as_ref().unwrap()).unwrap() > &1 {
                    // println!(
                    //     "elf couldn't move to their proposed location, remaining at {:?}",
                    //     &elf
                    // );
                    elves.push(elf.clone());
                } else {
                    // println!("yay, elf can move to their proposed location");
                    moved += 1;
                    elves.push(target.as_ref().unwrap().clone());
                }
            });

        // println!("new state\n\n{:?}\n\n", &elves);
        dir_priority.rotate_left(1);
        // println!("New dir priority = {:?}", dir_priority);
        println!(
            "** Round {} complete {}/{} elves moved **",
            round, moved, num_elves
        );
        if moved == 0 {
            println!("part 2 answer: {}", round);
            return Some(round);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
