use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|n| n.parse::<i64>().unwrap()).collect()
    // [1, 2, -3, 3, -2, 0, 4].to_vec()
}

// not mine, credit to https://gist.github.com/samueltardieu/3c80720af87d78e5b034afdb57d147b0
// but it doesn't come up with the right answer for the input set =/
fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

pub fn part_one(input: &str) -> Option<i64> {
    let values = parse(input);
    let mut values = values
        .iter()
        .map(|v| *v)
        .enumerate()
        .collect::<VecDeque<_>>();

    for i in 0..values.len() {
        // find the position of the item in the current list by using the original index
        let idx = values.iter().enumerate().position(|(j, _)| i == j).unwrap();

        // rotate so that it's at the beginning of the list
        values.rotate_left(idx);
        // pop the (orig pos, value) list off
        let (j, val) = values.pop_front().unwrap();
        // figure out number of places it should be rotated
        let d = val.rem_euclid(values.len() as i64) as usize;
        // rotate to that position
        values.rotate_left(d);
        // put the (orig pos, value) tuple back into the list
        values.push_front((j, val));
    }
    // now that it's all rotated, find where our 0 went
    let idx = values
        .iter()
        .enumerate()
        .position(|(_, (_, v))| (*v == 0))
        .unwrap();
    Some(
        values[(1000 + idx) % values.len()].1
            + values[(2000 + idx) % values.len()].1
            + values[(3000 + idx) % values.len()].1,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
