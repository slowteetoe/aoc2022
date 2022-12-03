pub fn part_one(input: &str) -> Option<u32> {
    let mut most = 0u32;
    let mut this_elf = 0u32;
    for l in input.lines() {
        if l.is_empty() {
            if this_elf > most {
                most = this_elf;
            }
            // println!("next elf");
            this_elf = 0;
            continue;
        }
        this_elf += l.parse::<u32>().unwrap();
        // println!("this elf now carrying: {:?} calories", this_elf);
    }
    Some(most)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: [u32; 3] = [0; 3]; // and we'll just keep this ordered
    let mut this_elf = 0u32;
    for l in input.lines() {
        if l.is_empty() {
            // update
            if this_elf > result[0] {
                result[2] = result[1];
                result[1] = result[0];
                result[0] = this_elf;
            } else if this_elf > result[1] {
                result[2] = result[1];
                result[1] = this_elf;
            } else if this_elf > result[2] {
                result[2] = this_elf;
            }
            this_elf = 0;
            continue;
        }
        this_elf += l.parse::<u32>().unwrap();
    }
    Some(result[0] + result[1] + result[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(41000));
    }
}
