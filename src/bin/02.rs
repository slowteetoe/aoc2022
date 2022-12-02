pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()
    .map(|round| {
        // TODO enums? state machine? something more elegant
        // println!("round: {:?}", round);
        match round.trim() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("invalid input"),
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines()
    .map(|round| {
        // X = lose 0, Y = draw 3, Z = win 6
        // rock = 1, paper = 2, scissors = 3
        match round.trim() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!("invalid input"),
        }
    }).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
