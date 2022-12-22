use std::collections::VecDeque;

pub fn parse(input: &str) -> VecDeque<isize> {
    input.lines().map(|n| n.parse::<isize>().unwrap()).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

pub fn part_one(input: &str) -> Option<u32> {
    let initial_ordering = parse(input);
    let mut working = initial_ordering.clone();
    for n in initial_ordering {
        if n == 0 {
            println!("skipping 0, it's a noop");
            continue;
        }
        let initial_pos = working.iter().position(|val| *val == n).unwrap();
        println!(
            "Found {} at position {} in the working array",
            n, initial_pos
        );
        println!("Rotating {} positions", n.abs());

        if n.is_positive() {
            for curr in 0..n.abs() {
                swap_right(&mut working, curr, curr + 1);
            }
        } else {
            let mut wrap = 0;
            for curr in 0..n.abs() {
                swap_left(
                    &mut working,
                    initial_pos as isize - curr + wrap,
                    initial_pos as isize - curr - 1 + wrap,
                );
                // if we wrapped, we have to offset that specialness
                if initial_pos as isize - curr == 0 {
                    wrap -= 1;
                }
            }
        }
        println!("** AFTER MOVEMENT **\n\t{:?}", &working);
    }

    None
}

pub fn swap_left(arr: &mut VecDeque<isize>, from: isize, to: isize) {
    println!("swapping position arr[{}] and arr[{}]", from, to);
    let array_len = arr.len();
    // deal with positive pos by wrapping around the array
    let from = if from < 0 {
        (array_len).checked_add_signed(from).unwrap() as usize
    } else {
        from as usize
    };
    let to = if to < 0 {
        (array_len).checked_add_signed(to).unwrap() as usize
    } else {
        to as usize
    };
    println!(
        "<-- swapping translated position arr[{}] and arr[{}]",
        from, to
    );

    if to == arr.len() - 1 {
        println!("oh so special");
        // ugh.  if we're going to wrap, we don't actually want to swap
        let last_val = arr[array_len - 1];

        let head = arr.pop_front().unwrap();
        println!("HEAD: {:?}", head);
        arr.push_back(head); // put the one we popped off back on the list, then swap the last two values
        arr.make_contiguous();
        arr[array_len - 2] = head;
        arr[array_len - 1] = last_val;
    } else {
        let tmp = arr[to];
        arr.make_contiguous();
        arr[to] = arr[from];
        arr[from] = tmp;
    }

    println!("new result: {:?}", &arr);
}

pub fn swap_right(arr: &mut VecDeque<isize>, from: isize, to: isize) {
    println!("--> swapping position arr[{}] and arr[{}]", from, to);
    // deal with negative pos by wrapping around the array
    let array_len = arr.len() as isize;
    let from = if from == array_len - 1 {
        // going to wrap to the front
        array_len - 1
    } else {
        from
    };
    let to = if to < 0 {
        (array_len).checked_add_signed(to).unwrap() as usize
    } else {
        to as usize
    };
    println!("swapping translated position arr[{}] and arr[{}]", from, to);

    arr.make_contiguous();
    let tmp = arr[to];
    arr[to] = arr[from];
    arr[from] = tmp;
    println!("new result: {:?}", &arr);
}

pub fn swap(arr: &mut VecDeque<isize>, from: usize, to: usize) {
    let tmp = arr[to];
    arr[to] = arr[from];
    arr[from] = tmp;
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
