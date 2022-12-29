use std::{
    cmp,
    collections::{BTreeMap, HashMap},
};

use regex::Regex;

#[derive(Debug)]
pub struct Valve {
    flow: i64,
    mask: i64,
    tunnels: Vec<String>,
}

pub type Cave = BTreeMap<String, Valve>;

pub fn parse(input: &str) -> Cave {
    let re = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
        .unwrap();
    let mut cave = BTreeMap::new();
    let mut n = 0u32;
    input.lines().for_each(|line| {
        let cap = re.captures(line).expect("unmatched line for regex");
        let name = cap[1].to_string();
        let flow = cap[2].parse::<i64>().expect("flow rate wrong");
        let tunnels: Vec<String> = cap[3]
            .to_string()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        cave.insert(
            name,
            Valve {
                flow,
                mask: i64::pow(2, n),
                tunnels,
            },
        );
        n += 1;
    });
    cave
}

fn calculate_distances(cave: &Cave) -> BTreeMap<(String, String), i64> {
    let mut distances = BTreeMap::new();
    cave.keys().for_each(|x| {
        cave.keys().for_each(|y| {
            // we'll generate all the values, e.g. AA -> AA to make the rest easier
            println!("building route from {} to {}", x, y);
            let dist = if cave.get(x).unwrap().tunnels.contains(y) {
                1
            } else {
                i64::MAX
            };
            distances.entry((x.clone(), y.clone())).or_insert(dist);
        });
    });
    cave.keys().for_each(|k| {
        cave.keys().for_each(|i| {
            cave.keys().for_each(|j| {
                println!("{} to {}", &i, &j);
                let ij = &*distances.get(&(i.clone(), j.clone())).unwrap();
                println!("{} to {}", &i, &k);
                let ik = &*distances.get(&(i.clone(), k.clone())).unwrap();
                println!("{} to {}", &k, &j);
                let kj = &*distances.get(&(k.clone(), j.clone())).unwrap();
                println!("now checking for next hop: {}, {}, {}", ij, ik, kj);
                let tmp = if *ik == i64::MAX || *kj == i64::MAX {
                    i64::MAX
                } else {
                    cmp::max(*ij, *ik + *kj)
                };
                distances.insert((i.clone(), j.clone()), tmp);
            });
        });
    });

    distances
}

pub fn visit(
    valve: String,
    budget: i64,
    state: i64,
    cave: &Cave,
    distances: &BTreeMap<(String, String), i64>,
    flow: i64,
    acc: &mut HashMap<i64, i64>,
) {
    let n = *acc.entry(state).or_default();
    acc.insert(state, cmp::max(n, flow));
    for k in cave.iter().filter(|(_, cv)| cv.flow > 0).map(|(ck, _)| ck) {
        let dist = distances.get(&(valve.clone(), k.to_string())).unwrap();
        let new_budget = budget - dist - 1;
        let mask = cave.get(k).unwrap().mask;
        if state & mask != 0 || new_budget < 0 {
            continue;
        } else {
            let flow_to_here = cave.get(k).unwrap().flow;
            visit(
                k.clone(),
                new_budget,
                state | mask,
                cave,
                distances,
                flow + new_budget + flow_to_here,
                acc,
            );
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let cave = parse(input);
    let distances = calculate_distances(&cave);
    dbg!(&distances);
    let mut acc: HashMap<i64, i64> = HashMap::new();
    let state = 0i64;
    visit(
        String::from("AA"),
        30i64,
        state,
        &cave,
        &distances,
        0,
        &mut acc,
    );
    dbg!(cave, &acc);
    // Some(*acc.values().max().unwrap())
    // Yeah, this doesn't work yet
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_one() {
        let _input = advent_of_code::read_file("examples", 16);
        // assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let _input = advent_of_code::read_file("examples", 16);
        // assert_eq!(part_two(&input), None);
    }
}
