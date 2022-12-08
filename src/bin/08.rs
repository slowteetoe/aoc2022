use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Tree {
    pub height: u32,
}

impl Tree {
    pub fn new(c: char) -> Self {
        Tree {
            height: c.to_digit(10).unwrap(),
        }
    }
}

pub fn build_grid(input: &str) -> (Vec<Vec<Tree>>, usize, usize) {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(Tree::new(c));
        }
        grid.push(row);
    }
    let num_rows = grid.len();
    let num_cols = grid.get(0).unwrap().len();
    return (grid, num_rows, num_cols);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, num_rows, num_cols) = build_grid(input);
    // now determine what's visible, we'll be naive for now and see how slow this is with our limited data set 100x100
    // ...aaaaaand it's <10ms so we'll just stick with this approach
    let mut visible = BTreeSet::new();

    for (y, row) in grid.iter().enumerate() {
        'outer: for (x, col) in row.iter().enumerate() {
            let coord = (x, y);
            if x == 0 || y == 0 || x == num_cols - 1 || y == num_rows - 1 {
                // these are on a border, and visible by definition
                visible.insert(coord);
            } else {
                // check visibility left
                for dx in (0..x).rev() {
                    if col.height <= row.get(dx)?.height {
                        break;
                    }
                    if dx == 0 {
                        // println!("{:?} is visible from the left (h={:?})", coord, col.height);
                        visible.insert(coord);
                        continue 'outer;
                    }
                }
                // right
                for dx in x + 1..num_cols {
                    if col.height <= row.get(dx)?.height {
                        break;
                    }
                    if dx == num_cols - 1 {
                        visible.insert(coord);
                        continue 'outer;
                    }
                }
                // from the top
                for dy in (0..y).rev() {
                    if col.height <= grid.get(dy)?.get(x)?.height {
                        break;
                    }
                    if dy == 0 {
                        visible.insert(coord);
                        continue 'outer;
                    }
                }
                // from the bottom
                for dy in y + 1..num_rows {
                    if col.height <= grid.get(dy)?.get(x)?.height {
                        break;
                    }
                    if dy == num_rows - 1 {
                        visible.insert(coord);
                        continue 'outer;
                    }
                }
            }
        }
    }
    Some(visible.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
