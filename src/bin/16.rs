use std::collections::BTreeMap;

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

pub fn part_one(input: &str) -> Option<u32> {
    let caves = parse(input);
    dbg!(caves);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
