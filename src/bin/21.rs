use std::collections::BTreeMap;

use itertools::Itertools;
use regex::Regex;

// let's try the simplest thing that could work first - couple of maps, one for resolved keys and one for pending
pub fn parse(input: &str) -> (BTreeMap<String, usize>, BTreeMap<String, String>) {
    let mut resolved = BTreeMap::new();
    let mut pending = BTreeMap::new();
    input
        .lines()
        .flat_map(|line| line.splitn(2, ":"))
        .tuples()
        .for_each(|(key, value)| {
            let value = value.trim();
            if value.chars().next().unwrap().is_digit(10) {
                resolved.insert(key.to_string(), value.parse::<usize>().unwrap());
            } else {
                pending.insert(key.to_string(), value.to_string());
            }
        });
    (resolved, pending)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut resolved, mut pending) = parse(input);
    for round in 0..128 {
        let mut remove = vec![];
        for (k, v) in pending.iter_mut() {
            // split value on spaces, we'll end up with monkey op monkey
            let (a, op, b) = v.splitn(3, " ").next_tuple().unwrap();
            let a_monkey = resolved.get(a);
            let b_monkey = resolved.get(b);
            match (a_monkey, b_monkey) {
                (Some(a), Some(b)) => {
                    println!("Resolved {}", v);
                    let result = match op {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => unreachable!(),
                    };
                    resolved.insert(k.to_string(), result);
                    remove.push(k.clone());
                }
                _ => (),
            }
        }
        for key in remove {
            pending.remove(&key);
        }
        println!(
            "After {} round(s): pending {:?} resolved {:?}",
            round,
            pending.len(),
            resolved.len()
        );
        if resolved.contains_key("root") {
            return Some(resolved.get("root").unwrap().clone());
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
