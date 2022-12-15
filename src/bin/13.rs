use core::fmt;
use std::cell::RefCell;

use itertools::EitherOrBoth::*;
use itertools::Itertools;

// why not, let's get complicated and make a list structure!
type LvalChildren = Vec<Box<Lval>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Lval {
    Tombstone,
    Num(u8),
    List(LvalChildren),
}

#[derive(Debug)]
pub struct Packet(Vec<Box<Lval>>);

#[derive(Debug)]
pub struct PacketPair {
    p1: Packet,
    p2: Packet,
}

impl PacketPair {
    pub fn in_order(&self) -> bool {
        println!("comparing {:?} and {:?}", self.p1.0, self.p2.0);
        // Itertools zip_longest is pretty useful for this!
        for pair in self.p1.0.iter().zip_longest(self.p2.0.iter()) {
            match pair {
                Both(left, right) => {
                    println!("comparing {:?} and {:?}", left, right);
                    if !in_order(&**left, &**right) {
                        println!("*** packets NOT in order ***");
                        return false;
                    }
                }
                Left(_) => {
                    println!("right ran out of elements, inputs NOT in right order");
                    return false;
                }
                Right(_) => {}
            }
        }
        println!("*** packets in order ***");
        true
    }
}

fn in_order(left: &Lval, right: &Lval) -> bool {
    match (left, right) {
        (Lval::Num(l), Lval::Num(r)) => {
            if l > r {
                println!("left num is larger than right num, not in order");
                return false;
            }
        }
        (Lval::List(left), Lval::List(right)) => {
            if left.len() > right.len() {
                println!("right list is shorter, so can't be in order");
                return false;
            }
            // make a recursive call
            let result = lists_in_order(left, right);
            if result == false {
                println!("lists weren't in order");
                return false;
            }
        }
        (Lval::Num(l), Lval::List(r)) => {
            println!("make left a list and call compare on the two");
            if !r.is_empty() {
                let rval = &**r.get(0).unwrap();
                match *rval {
                    Lval::Num(n) => {
                        if *l > n {
                            println!("not in order, left num is > val in right list");
                            return false;
                        }
                    }
                    _ => {
                        unreachable!("shouldn't have hit this scenario, rval was {:?}", rval);
                    }
                }
            }
        }
        (Lval::List(l), Lval::Num(r)) => {
            println!("make right a list and call compare on the two");
            if !l.is_empty() {
                let lval = &**l.get(0).unwrap();
                match *lval {
                    Lval::Num(n) => {
                        if n > *r {
                            println!("not in order, val in left list is > right num");
                            return false;
                        }
                    }
                    _ => {
                        unreachable!("shouldn't have hit this scenario");
                    }
                }
            }
        }
        _ => unreachable!(""),
    }
    true
}

fn lists_in_order(left: &Vec<Box<Lval>>, right: &Vec<Box<Lval>>) -> bool {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            Both(left, right) => {
                println!("comparing {:?} and {:?}", left, right);
                if !in_order(&**left, &**right) {
                    println!("*** packets NOT in order ***");
                    return false;
                }
            }
            Left(_) => {
                println!("right ran out of elements, inputs NOT in right order");
                return false;
            }
            Right(_) => {}
        }
    }
    println!("lists_in_order still LIES?");
    true
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lval::Num(n) => write!(f, "{}", n),
            Lval::List(children) => write!(f, "({})", rec_print(&children)),
            Lval::Tombstone => write!(f, "🪦"),
        }
    }
}

impl Packet {
    pub fn parse(p: &str) -> Packet {
        let mut stack = vec![];
        let mut thisvec = RefCell::new(Vec::new());

        for ch in p.chars() {
            // println!("looking at {}, stack currently {:?}", ch, &stack);
            match ch {
                '[' => stack.push(Box::new(Lval::Tombstone)),
                '0'..='9' => {
                    stack.push(Box::new(Lval::Num(ch.to_string().parse::<u8>().unwrap())));
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
                        match *item {
                            Lval::List(contents) => {
                                thisvec.borrow_mut().push(Box::new(Lval::List(contents)));
                            }
                            Lval::Num(_) => {
                                thisvec.borrow_mut().push(item);
                            }
                            Lval::Tombstone => {
                                // see what we have, push back onto the stack as a list
                                if stack.is_empty() {
                                    // we're at the last tombstone, so it's complete
                                    let mut result = thisvec.get_mut().to_vec();
                                    result.reverse();
                                    return Packet(result);
                                } else {
                                    // println!("done popping stack, pushing the list we just built up back on");
                                    let mut tmp = thisvec.get_mut().to_vec();
                                    tmp.reverse();
                                    stack.push(Box::new(Lval::List(tmp)));
                                }
                                thisvec = RefCell::new(Vec::new());
                                break;
                            }
                        }
                    }
                }
                ',' => {
                    continue;
                }
                _ => unreachable!("invalid chars in input"),
            }
        }
        unreachable!("unbalanced input if we see this");
    }

    pub fn list_of(stuff: Vec<Lval>) -> Box<Lval> {
        let mut children = vec![];
        for s in stuff {
            children.push(Box::new(s))
        }
        Box::new(Lval::List(children))
    }

    pub fn lval_num(n: u8) -> Box<Lval> {
        Box::new(Lval::Num(n))
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = vec![];
        for expr in &self.0 {
            match *expr.clone() {
                Lval::Num(n) => {
                    result.push(format!("{}", n));
                }
                Lval::List(children) => {
                    result.push(format!("({})", rec_print(&children)));
                }
                Lval::Tombstone => {
                    result.push(String::from("🪦"));
                }
            }
        }
        write!(f, "{}", result.join(","))
    }
}

fn rec_print(children: &[Box<Lval>]) -> String {
    let mut resp = vec![];
    for c in children {
        resp.push(format!("{}", c));
    }
    resp.join(",")
}

impl fmt::Display for PacketPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1: [{}], p2: [{}]", &self.p1, &self.p2)
    }
}

pub fn parse_packets(input: &str) -> Vec<PacketPair> {
    input
        .lines()
        .chain(vec!["\n\n"])
        .tuples()
        .map(|(p1, p2, _whitespace)| PacketPair {
            p1: Packet::parse(p1),
            p2: Packet::parse(p2),
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let packets = &parse_packets(&input);
    for p in packets {
        println!("{}", &p);
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
}
