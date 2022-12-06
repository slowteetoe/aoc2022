use ringbuffer::*;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    find_n_unique(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_n_unique(input, 14)
}

pub fn find_n_unique(input: &str, n: usize) -> Option<usize> {
    let mut buffer = AllocRingBuffer::new();
    for (idx, c) in input.char_indices() {
        buffer.push(c);
        if idx > n {
            // check if the buffer is completely unique
            let mut checkmap = HashSet::new();
            // since we can't set ringbuffer to the size we want (capacity must be power of 2), we need to lookback n
            for i in 1..=n {
                checkmap.insert(buffer.get(-1 * i as isize));
            }
            if checkmap.len() == n {
                // if we have n unique values, just return after correcting to 1-based index
                return Some(idx + 1);
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
