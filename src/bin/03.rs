use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|sack| {
            // Why not just use a hashmap?
            // I was expecting part 2 to be tricky, but it wasn't.  This made part one way more difficult that it needed to be - do the simplest thing that works next time!
            let mut arr: [u32; 52] = [0; 52];

            for c in sack[0..sack.len() / 2].chars() {
                arr[char_to_index(c) as usize] = 1;
            }

            for c in sack[sack.len() / 2..].chars() {
                let idx = char_to_index(c) as usize;
                if arr[idx] == 1 {
                    arr[idx] = 2;
                }
            }

            let mut priority = 0u32;
            for i in 0..arr.len() {
                if arr[i] >= 2 {
                    priority += i as u32 + 1;
                }
            }
            priority
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut priority = 0u32;
    // TODO figure out how Itertools tuples SHOULD be used... this is nice, but doesn't seem quite right
    let mut it = input.lines().tuples::<(&str, &str, &str)>();
    while let tup = it.next() {
        if tup.is_none() {
            break;
        }
        let (a, b, c) = tup.unwrap();

        let a_sack = allocate_sack(a);
        let b_sack = allocate_sack(b);
        let c_sack = allocate_sack(c);

        a_sack.keys().for_each(|k| {
            if b_sack.contains_key(k) && c_sack.contains_key(k) {
                priority += char_to_priority(*k);
            }
        });
    }
    Some(priority)
}

fn allocate_sack(sack: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in sack[0..sack.len()].chars() {
        map.insert(c, 1);
    }
    map
}

// a-z = 0..26, A-Z = 26..52
fn char_to_index(c: char) -> u32 {
    match c.is_ascii_lowercase() {
        true => c as u32 - 97,
        false => c as u32 - 65 + 26, // want uppercase to slot into 26-51 to make computing priority easier
    }
}

fn char_to_priority(c: char) -> u32 {
    char_to_index(c) + 1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_priorities() {
        assert_eq!(18, char_to_priority('r'));
        assert_eq!(52, char_to_priority('Z'));
    }
}
