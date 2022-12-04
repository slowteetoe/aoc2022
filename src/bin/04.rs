use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines() // "2-4,6-8"
            .flat_map(|line| line.split(',').flat_map(|s| s.split('-'))) // "2-4" to "2" "4"
            .map(|n| n.parse::<u8>().unwrap()) // 2
            .tuples::<(u8, u8, u8, u8)>()
            .filter(|(a1, a2, b1, b2)| a1 >= b1 && a2 <= b2 || b1 >= a1 && b2 <= a2)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| line.split(',').flat_map(|s| s.split('-')))
            .map(|n| n.parse::<u8>().unwrap())
            .tuples::<(u8, u8, u8, u8)>()
            .filter(|(a1, a2, b1, b2)| !(a2 < b1 || b2 < a1))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
