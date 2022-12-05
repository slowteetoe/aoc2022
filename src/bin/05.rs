use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let mut crate_data = vec![];
    let mut crates: Vec<Vec<char>>;
    let mut moves = vec![];

    let mut count = 0;
    let mut parsing_crates = true;

    for line in input.lines() {
        if line == "" {
            continue;
        } else if line.contains(" 1   ") {
            // capture down to 1 2 ... n
            parsing_crates = false;
            count = (line.len() + 1) / 4;
        } else if parsing_crates {
            crate_data.push(line);
        } else {
            moves.push(line);
        }
    }
    // then reverse and loop to build up each columns boxes
    crate_data.reverse();
    crates = vec![vec![]; count];
    for line in crate_data {
        for (idx, c) in line.char_indices() {
            if idx > 0 && (idx - 1) % 4 == 0 && c != ' ' {
                // indexes are 2 6 10 etc but zero based
                crates[(idx - 1) / 4].push(c);
            }
        }
    }

    // now process the moves on the columns of boxes
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in moves {
        let cap = re.captures(line).unwrap();
        let num = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let val = crates[from].pop().unwrap();
            crates[to].push(val);
        }
    }

    // finally, grab the last crate in each column
    let mut result = String::new();
    for col in 0..crates.len() {
        result.push(crates[col][crates[col].len() - 1]);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    // no refactoring here, just C+P from part one!
    let mut crate_data = vec![];
    let mut crates: Vec<Vec<char>>;
    let mut moves = vec![];

    let mut count = 0;
    let mut parsing_crates = true;

    for line in input.lines() {
        if line == "" {
            continue;
        } else if line.contains(" 1   ") {
            // capture down to 1 2 ... n
            parsing_crates = false;
            count = (line.len() + 1) / 4;
        } else if parsing_crates {
            crate_data.push(line);
        } else {
            moves.push(line);
        }
    }
    // then reverse and loop to build up each columns boxes
    crate_data.reverse();
    crates = vec![vec![]; count];
    for line in crate_data {
        for (idx, c) in line.char_indices() {
            if idx > 0 && (idx - 1) % 4 == 0 && c != ' ' {
                // indexes are 2 6 10 etc but zero based
                crates[(idx - 1) / 4].push(c);
            }
        }
    }

    // now process the moves on the columns of boxes
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in moves {
        let cap = re.captures(line).unwrap();
        let num = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;

        let mut tmp = vec![];
        for _ in 0..num {
            tmp.push(crates[from].pop().unwrap());
        }
        tmp.reverse();
        crates[to].extend_from_slice(&tmp);
        // crates[to].extend_from_slice(crates[from][num - 1..crates[from].len()]);
    }

    // finally, grab the last crate in each column
    let mut result = String::new();
    for col in 0..crates.len() {
        result.push(crates[col][crates[col].len() - 1]);
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
