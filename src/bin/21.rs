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
    for _round in 0..128 {
        let mut remove = vec![];
        for (k, v) in pending.iter_mut() {
            // split value on spaces, we'll end up with monkey op monkey
            let (a, op, b) = v.splitn(3, " ").next_tuple().unwrap();
            let a_monkey = resolved.get(a);
            let b_monkey = resolved.get(b);
            match (a_monkey, b_monkey) {
                (Some(a), Some(b)) => {
                    // println!("Resolved {}", v);
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
        // println!(
        //     "After {} round(s): pending {:?} resolved {:?}",
        //     round,
        //     pending.len(),
        //     resolved.len()
        // );
        if resolved.contains_key("root") {
            return Some(resolved.get("root").unwrap().clone());
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut resolved, mut pending) = parse(input);
    resolved.remove("humn");
    for _round in 0..256 {
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
    }
    // we're looking for the path from humn -> root and then we'll have to build up the answer... let's see how this goes
    println!(
        "After the resolving rounds: pending({:?})\n{:?}",
        pending.len(),
        pending
    );
    let mut mixed = BTreeMap::new();
    // let's make one more pass through the pending, this time just replacing what we can.  From this point on, we'll have mixed expressions
    for (k, v) in pending.iter() {
        let mut tmp = String::from("");
        let (a, op, b) = v.splitn(3, " ").next_tuple().unwrap();
        if pending.contains_key(a) {
            tmp.push_str(" (");
            tmp.push_str(pending.get(a).unwrap());
            tmp.push_str(")");
        } else if resolved.contains_key(a) {
            tmp.push_str(&resolved.get(a).unwrap().to_string());
        } else {
            tmp.push_str("stoopidhuman");
        }

        tmp.push_str(" ");
        tmp.push_str(op);
        tmp.push_str(" ");

        if pending.contains_key(b) {
            tmp.push_str(" (");
            tmp.push_str(pending.get(b).unwrap());
            tmp.push_str(")");
        } else if resolved.contains_key(b) {
            tmp.push_str(&resolved.get(b).unwrap().to_string());
        } else {
            tmp.push_str("stoopidhuman");
        }
        mixed.insert(k.clone(), tmp);
    }

    // dbg!(&mixed);

    let root = mixed.get("root").unwrap();

    // now let's replace until we don't replace any more
    let re = Regex::new(r".*?(\w{4})+.*?").unwrap();

    dbg!(&mixed);

    loop {
        let caps: Vec<regex::Captures> = re.captures_iter(root).collect();
        println!("captured: {}", caps.len());
        for c in caps.iter() {
            let key = c.get(1).unwrap().as_str();
            println!("{:?}", key);
            let rep = match mixed.get(key) {
                Some(val) => val,
                None => key,
            };
            println!("cap[{}] = shoudl be replaced with {:?}", key, rep);
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    // advent_of_code::solve!(2, part_two, input);
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
        let _input = advent_of_code::read_file("examples", 21);
        // assert_eq!(part_two(&input), Some(301));
    }
}
