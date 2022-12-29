use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, HashMap},
};

use itertools::Itertools;
use tracing::{info, span, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i32, i32);

// I don't think I want a grid for part one, let's just work with points
pub fn parse(input: &str) -> BTreeSet<Point> {
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
        .collect()
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
    // we're at 9-10s for part one right now...
    // let's try this a different way, we'll create a vec with 8 elements, then store the indexes we need to check for each direction in each element
    // NOPE: no faster
    let mut dir_priority = vec![
        // still N,S,W,E
        ((0, -1), (0, 1, 2)),
        ((0, 1), (5, 6, 7)),
        ((-1, 0), (0, 3, 5)),
        ((1, 0), (2, 4, 7)),
    ];
    // move these out of loop to avoid re-alloc, we're going to hit every cell every round anyhow
    let mut nearby = vec![false; 8];
    let mut proposals = vec![(Point(0, 0), None); elves.len()]; // elf at Point is going to want to move to Point

    for round in 1..=10 {
        let _span_ = span!(Level::TRACE, "round", round).entered();

        // Proposal phase
        let ps = span!(
            Level::TRACE,
            "proposal phase, num_elves",
            num_elves = elves.len()
        )
        .entered();
        info!("start");
        'nextelf: for (idx, elf) in elves.iter().enumerate() {
            nearby[0] = elves.contains(&Point(elf.0 - 1, elf.1 - 1));
            nearby[1] = elves.contains(&Point(elf.0, elf.1 - 1));
            nearby[2] = elves.contains(&Point(elf.0 + 1, elf.1 - 1));
            nearby[3] = elves.contains(&Point(elf.0 - 1, elf.1));
            nearby[4] = elves.contains(&Point(elf.0 + 1, elf.1));
            nearby[5] = elves.contains(&Point(elf.0 - 1, elf.1 + 1));
            nearby[6] = elves.contains(&Point(elf.0, elf.1 + 1));
            nearby[7] = elves.contains(&Point(elf.0 + 1, elf.1 + 1));

            if nearby.iter().fold(false, |acc, n| acc || *n) == false {
                // println!("no nearby elves, not going anywhere");
                proposals[idx] = (elf.clone(), None);
                continue;
            }

            // check in order for the different directions
            for (dxdy, cardinal) in dir_priority.iter() {
                if nearby[cardinal.0] || nearby[cardinal.1] || nearby[cardinal.2] {
                    // println!("elf in the way, checking a different direction");
                    continue;
                } else {
                    let proposal = Point(elf.0 + dxdy.0, elf.1 + dxdy.1);
                    // println!("elf at {:?} proposes moving to {:?}", &elf, &proposal);
                    proposals[idx] = (elf.clone(), Some(proposal));
                    continue 'nextelf;
                }
            }
            proposals[idx] = (elf.clone(), None);
        }
        info!("end");
        ps.exit();

        let span = span!(Level::INFO, "move round").entered();
        info!("start");
        // Move phase
        elves.clear();

        // resolve the proposals (and who stayed in place)
        let did_not_move = proposals
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

        elves = did_not_move.iter().map(|e| e.clone()).collect();

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

        proposals
            .iter()
            .filter(|(_, target)| target.is_some())
            .for_each(|(elf, target)| {
                if collisions.get(target.as_ref().unwrap()).unwrap() > &1 {
                    elves.insert(elf.clone());
                } else {
                    elves.insert(target.as_ref().unwrap().clone());
                }
            });
        info!("end");
        span.exit();
        dir_priority.rotate_left(1);
        info!("round complete");
    }

    // find furthest N E S W elves, this makes up the final grid dimensions to calculate
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = parse(input);
    let mut dir_priority = vec![Point(0, -1), Point(0, 1), Point(-1, 0), Point(1, 0)];
    let vectors = vectors();

    // let num_elves = elves.len();

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
        let did_not_move = proposals
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

        elves = did_not_move.iter().map(|e| e.clone()).collect();

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
                    elves.insert(elf.clone());
                } else {
                    // println!("yay, elf can move to their proposed location");
                    moved += 1;
                    elves.insert(target.as_ref().unwrap().clone());
                }
            });

        // println!("new state\n\n{:?}\n\n", &elves);
        dir_priority.rotate_left(1);
        // println!("New dir priority = {:?}", dir_priority);
        // println!(
        //     "** Round {} complete {}/{} elves moved **",
        //     round, moved, num_elves
        // );
        if moved == 0 {
            // println!("part 2 answer: {}", round);
            return Some(round);
        }
    }
    None
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::WARN) // switch to TRACE to see all output
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
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
