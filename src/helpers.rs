/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn manhattan(p1: (i64, i64), p2: (i64, i64)) -> usize {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as usize
}

pub fn intersection(
    line1: ((i64, i64), (i64, i64)),
    line2: ((i64, i64), (i64, i64)),
) -> Option<(i64, i64)> {
    let (start1, end1) = line1;
    let (start2, end2) = line2;

    let a1 = end1.1 - start1.1;
    let b1 = start1.0 - end1.0;
    let c1 = a1 * start1.0 + b1 * start1.1;

    let a2 = end2.1 - start2.1;
    let b2 = start2.0 - end2.0;
    let c2 = a2 * start2.0 + b2 * start2.1;

    let delta = a1 * b2 - a2 * b1;

    if delta == 0 {
        return None;
    }

    Some(((b2 * c1 - b1 * c2) / delta, (a1 * c2 - a2 * c1) / delta))
}
