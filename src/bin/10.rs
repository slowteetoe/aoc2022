pub fn part_one(input: &str) -> Option<i32> {
    let mut raw = vec![];
    for instruction in input.lines() {
        if instruction.starts_with("addx ") {
            let val = instruction.replace("addx ", "").parse::<i32>().unwrap();
            // take 2 cycles to complete
            raw.push(0);
            raw.push(val);
        } else if instruction.starts_with("noop") {
            // takes 1 cycle to complete
            raw.push(0);
        }
    }

    let mut signal_strength = 0i32;
    let mut register = 1i32;
    for (idx, x) in raw.iter().enumerate() {
        if (idx + 1) % 20 == 0 && idx > 0 {
            if idx + 1 == 20 || ((idx + 1) / 20) % 2 == 1 {
                // println!("x at {:?} is {:?}", &idx + 1, &register);
                signal_strength += register * (idx + 1) as i32;
            }
        }
        register += x;
    }
    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
