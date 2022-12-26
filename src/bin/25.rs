use bimap::{self, BiMap};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHARMAP: BiMap<char, i32> = {
        let mut m = BiMap::new();
        m.insert('=', -2);
        m.insert('-', -1);
        m.insert('0', 0);
        m.insert('1', 1);
        m.insert('2', 2);
        m
    };
}

pub fn part_one(input: &str) -> Option<String> {
    let decimal = input
        .lines()
        .flat_map(|s| convert_from_snafu(s).parse::<i128>())
        .sum::<i128>();

    Some(convert_to_snafu(&decimal.to_string()))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

// lets go recursive
pub fn convert_to_snafu(decimal: &str) -> String {
    // from right to left, 1s, 5s, 25s, 125s, etc
    // but digits are =, -, 0, 1, 2  (= worth -2, - worth -1)
    let decimal = decimal.parse::<isize>().unwrap();
    if decimal == 0 {
        return String::from("");
    }
    match decimal % 5 {
        remainder @ 0..=2 => [
            convert_to_snafu(&(decimal / 5).to_string()),
            remainder.to_string(),
        ]
        .join(""),
        3 => [
            convert_to_snafu(&(1 + decimal / 5).to_string()),
            String::from("="),
        ]
        .join(""),
        4 => [
            convert_to_snafu(&(1 + decimal / 5).to_string()),
            String::from("-"),
        ]
        .join(""),
        _ => unreachable!(),
    }
}

// return a decimal string, e.g. "12"
pub fn convert_from_snafu(snafu: &str) -> String {
    let len = snafu.len();
    snafu
        .char_indices()
        .map(|(idx, ch)| {
            5i128.pow((len - idx - 1) as u32) as i128 * *CHARMAP.get_by_left(&ch).unwrap() as i128
        })
        .sum::<i128>()
        .to_string()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    snafu_to_dec!(a, "1=-0-2", "1747");
    snafu_to_dec!(b, "12111", "906");
    snafu_to_dec!(c, "2=0=", "198");
    snafu_to_dec!(d, "21", "11");
    snafu_to_dec!(e, "2=01", "201");
    snafu_to_dec!(f, "111", "31");
    snafu_to_dec!(g, "20012", "1257");
    snafu_to_dec!(h, "112", "32");
    snafu_to_dec!(i, "1=-1=", "353");
    snafu_to_dec!(j, "1-12", "107");
    snafu_to_dec!(k, "12", "7");
    snafu_to_dec!(l, "1=", "3");
    snafu_to_dec!(m, "122", "37");

    dec_to_snafu!(one, "1", "1");
    dec_to_snafu!(two, "2", "2");
    dec_to_snafu!(three, "3", "1=");
    dec_to_snafu!(four, "4", "1-");
    dec_to_snafu!(five, "5", "10");
    dec_to_snafu!(six, "6", "11");
    dec_to_snafu!(seven, "7", "12");
    dec_to_snafu!(eight, "8", "2=");
    dec_to_snafu!(nine, "9", "2-");
    dec_to_snafu!(ten, "10", "20");
    dec_to_snafu!(eleven, "11", "21");
    dec_to_snafu!(twelve, "12", "22");
    dec_to_snafu!(thirteen, "13", "1==");
    dec_to_snafu!(fourteen, "14", "1=-");
    dec_to_snafu!(fifteen, "15", "1=0");
    dec_to_snafu!(twenty, "20", "1-0");

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        // assert_eq!(part_one(&input), Some(4890));
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }

    #[macro_export]
    macro_rules! snafu_to_dec {
        ($name:ident, $a:expr, $b:expr ) => {
            #[test]
            fn $name() {
                let result = convert_from_snafu($a);
                assert_eq!(result, $b);
            }
        };
    }

    #[macro_export]
    macro_rules! dec_to_snafu {
        ($name:ident, $a:expr, $b:expr ) => {
            #[test]
            fn $name() {
                let result = convert_to_snafu($a);
                assert_eq!(result, $b);
            }
        };
    }
}
