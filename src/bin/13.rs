use core::fmt;
use std::cell::RefCell;
use std::cmp::Ordering;

use itertools::EitherOrBoth::*;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Num(u8),
    List(Vec<Packet>),
    Tombstone, // this is awkward, but let's get it working before cleaning it up
}

pub struct Packets(Vec<Packet>, Vec<Packet>);

impl Packets {
    pub fn in_order(&self) -> bool {
        println!("comparing {:?} and {:?}", self.0, self.1);

        for pair in self.0.iter().zip_longest(self.1.iter()) {
            println!("{:?}", pair);
            match pair {
                Both(left, right) => {
                    println!("comparing {:?} and {:?}", left, right);
                    match in_order(left, right) {
                        Ordering::Less => return true,
                        Ordering::Greater => return false,
                        Ordering::Equal => {
                            println!("couldn't tell, continuing to process rules");
                        }
                    }
                }
                Left(_) => {
                    println!("right ran out of elements, inputs NOT in right order");
                    return false;
                }
                Right(_) => {
                    println!("left ran out of elements, inputs in right order");
                    return true;
                }
            }
        }
        false
    }

    // impl in_order2(&self) -> bool {
    //     println!("comparing {:?} and {:?}", self.0, self.1);

    //     for pair in self.0.iter().zip_longest(self.1.iter()) {
    //         match pair {
    //             Both(left, right) => {
    //                 // call is_this_in_order()
    //                 match (left, right) {
    //                     (Packet::Num(left), Packet::Num(right)) => {
    //                         if left == right {
    //                             // have to continue comparing, don't know if they're in order yet
    //                         } else {
    //                             return left < right;
    //                         }
    //                     }
    //                 }
    //             },
    //             Right(_) => {
    //                 true
    //             },
    //             Left(_) => {
    //                 false
    //             },
    //         }
    //     }
    // }
}

// so confused.  I guess we're really representing "decision made" true or false, or "undetermined" and we have to keep checking packet
fn in_order(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Num(left_num), Packet::Num(right_num)) => {
            // the question/examples are horribly written, but someone on reddit said that it helps to think of the numeric comparison as
            // alphabetic.  i.e. the 2 in [2, 0] is already greater than the 1 in [1, 9] so the packets are in order and you don't have to compare the rest
            if left_num == right_num {
                Ordering::Equal
            } else {
                left_num.cmp(right_num)
            }
        }
        (Packet::List(left), Packet::List(right)) => lists_in_order(left, right),
        (Packet::Num(l), Packet::List(r)) => {
            println!("make left a list and call compare on the two");
            // have to put right back into a list since we matched out of it
            lists_in_order(
                &vec![Packet::List(vec![Packet::Num(*l)])],
                &vec![Packet::List(r.to_vec())],
            )
        }
        (Packet::List(l), Packet::Num(r)) => {
            println!("make right a list and call compare on the two");
            lists_in_order(
                &vec![Packet::List(l.to_vec())],
                &vec![Packet::List(vec![Packet::Num(*r)])],
            )
        }
        _ => unreachable!(""),
    }
}

// get rid of the awkward Option(bool) and use Ordering
fn lists_in_order(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            Both(left, right) => {
                println!("comparing {:?} and {:?}", left, right);
                let result = in_order(left, right);
                match result {
                    Ordering::Equal => {
                        continue;
                    }
                    _ => {
                        return result;
                    }
                }
            }
            Left(_) => {
                println!("right LIST ran out of elements, inputs NOT in right order");
                return Ordering::Greater;
            }
            Right(_) => {
                println!("left LIST ran out of elements, input is in correct order");
                return Ordering::Less;
            }
        }
    }
    Ordering::Equal
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::Num(n) => write!(f, "{}", n),
            Packet::List(children) => write!(f, "({})", rec_print(&children)),
            Packet::Tombstone => Ok(()),
        }
    }
}

impl Packet {
    pub fn parse(p: &str) -> Vec<Packet> {
        let mut stack = vec![];
        let mut thisvec = RefCell::new(Vec::new());

        for ch in p.chars() {
            // println!("looking at {}, stack currently {:?}", ch, &stack);
            match ch {
                '[' => stack.push(Packet::Tombstone),
                '0'..='9' => {
                    stack.push(Packet::Num(ch.to_string().parse::<u8>().unwrap()));
                }
                ']' => {
                    // start unwinding until we hit a tombstone, reverse the vec and put it back on the stack
                    // println!("poppin like it's hot...");
                    loop {
                        let val = stack.pop();
                        if val.is_none() {
                            // hit the end of the stack, nothing left
                            unreachable!("shouldn't bottom out");
                        }
                        let item = val.unwrap();
                        match item {
                            Packet::List(contents) => {
                                thisvec.borrow_mut().push(Packet::List(contents));
                            }
                            Packet::Num(_) => {
                                thisvec.borrow_mut().push(item);
                            }
                            Packet::Tombstone => {
                                // see what we have, push back onto the stack as a list
                                if stack.is_empty() {
                                    // we're at the last tombstone, so it's complete
                                    let mut result = thisvec.get_mut().to_vec();
                                    result.reverse();
                                    return result;
                                } else {
                                    // println!("done popping stack, pushing the list we just built up back on");
                                    let mut tmp = thisvec.get_mut().to_vec();
                                    tmp.reverse();
                                    stack.push(Packet::List(tmp));
                                }
                                thisvec = RefCell::new(Vec::new());
                                break;
                            }
                        }
                    }
                }
                ',' => (),
                _ => unreachable!("invalid chars in input"),
            }
        }
        unreachable!("unbalanced input if we see this");
    }
}

fn rec_print(children: &[Packet]) -> String {
    let mut resp = vec![];
    for c in children {
        resp.push(format!("{}", c));
    }
    resp.join(",")
}

pub fn parse_packets(input: &str) -> Vec<Packets> {
    input
        .lines()
        .chain(vec!["\n\n"])
        .tuples()
        .map(|(p1, p2, _whitespace)| Packets(Packet::parse(p1), Packet::parse(p2)))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let packets = &parse_packets(&input);
    for p in packets {
        println!("{:?} {:?}", &p.0, &p.1);
    }

    let answer: usize = packets
        .iter()
        .enumerate()
        .map(|(idx, pair)| (idx, pair.in_order()))
        .filter(|m| m.1)
        .map(|m| m.0 + 1)
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_packet1() {
        let p = parse_packets(&String::from("[[1],[2,3,4]]\n[[1],4]"));
        assert_eq!(true, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet2() {
        let p = parse_packets(&String::from("[[1],[2,3,4]]\n[[1],4]"));
        assert_eq!(true, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet3() {
        let p = parse_packets(&String::from("[9]\n[[8,7,6]]"));
        assert_eq!(false, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet4() {
        let p = parse_packets(&String::from("[[4,4],4,4]\n[[4,4],4,4,4]"));
        assert_eq!(true, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet5() {
        let p = parse_packets(&String::from("[7,7,7,7]\n[7,7,7]"));
        assert_eq!(false, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet6() {
        let p = parse_packets(&String::from("[]\n[3]"));
        assert_eq!(true, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet7() {
        let p = parse_packets(&String::from("[[[]]]\n[[]]"));
        assert_eq!(false, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_packet8() {
        let p = parse_packets(&String::from(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ));
        assert_eq!(false, p.get(0).unwrap().in_order());
    }

    #[test]
    fn test_rando_packet() {
        // got this from reddit
        let p = parse_packets("[7,7,7]\n[7,7,7,[]]");
        assert_eq!(true, p.get(0).unwrap().in_order());
    }

    // [[1],[2,3,4]]\n[[1],2,3,4]
    #[test]
    fn test_rando_packet2() {
        // got this from reddit
        let p = parse_packets("[[1],[2,3,4]]\n[[1],2,3,4]");
        assert_eq!(false, p.get(0).unwrap().in_order());
    }
}
