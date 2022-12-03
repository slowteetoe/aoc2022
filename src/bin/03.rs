pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|sack| {
                let mut arr: [u32; 52] = [0; 52];
                let mut sack_idx = 0usize;
                sack.chars().for_each(|c| {
                    sack_idx += 1;
                    if c == '\n' {
                        return;
                    }
                    // ugh, index into the "right" slot
                    let idx = match c.is_ascii_lowercase() {
                        true => c as u32 - 97,
                        false => c as u32 - 65 + 26, // want uppercase to slot into 26-51 to make computing priority easier
                    };
                    if sack_idx < sack.len() / 2 + 1 {
                        // println!("{:?} goes into left compartment", c);
                        arr[idx as usize] = 1;
                    } else {
                        // println!("{:?} goes into right compartment", c);
                        if arr[idx as usize] == 1 {
                            arr[idx as usize] = 2;
                        }
                    }
                });

                let mut priority = 0u32;
                for i in 0..arr.len() {
                    if arr[i] >= 2 {
                        // println!("arr[{:?}] which should add {:?} to the total", i, i + 1);
                        priority += i as u32 + 1;
                    }
                }
                priority
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 3);
    //     assert_eq!(part_two(&input), None);
    // }
}
