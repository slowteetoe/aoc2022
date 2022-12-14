use core::fmt;

use itertools::Itertools;

// why not, let's get complicated
type LvalChildren = Vec<Box<Lval>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Lval {
    Num(u8),
    List(LvalChildren),
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lval::Num(n) => write!(f, "{}", n),
            Lval::List(children) => write!(f, "({})", rec_print(&children)),
        }
    }
}

// going to have a recursive datastructure so we'll need a Box
#[derive(Debug)]
pub struct Packet(Vec<Box<Lval>>);

impl Packet {
    // oh i bet theres a fun regex for this...
    // pub fn parse2(p: &str) -> Packet {
    //     println!("parsing {:?}", &p);

    //     let mut stack = vec![];

    //     for ch in p.chars() {
    //         match ch {
    //             '[' => stack.push(-1),
    //             ']' => {
    //                 let thisvec = vec![];
    //                 while let v = match stack.pop() {
    //                 Some(val) => if val == -1 { stack.push(vec![]) } else { stack.push},
    //                 None => unreachable!("well crap.")
    //                 };
    //             },
    //             ',' => todo!("ignore"),
    //             _ => todo!("convert to u8")
    //         }
    //     }
    //     Packet(vec![])
    // }

    pub fn parse(p: &str) -> Packet {
        Packet(vec![
            Self::lval_num(6),
            Self::listOf(vec![Lval::Num(7), Lval::Num(8)]),
            Self::listOf(vec![
                *Self::listOf(vec![Lval::Num(9)]),
                *Self::listOf(vec![]),
            ]),
        ])
    }

    pub fn listOf(stuff: Vec<Lval>) -> Box<Lval> {
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

#[derive(Debug)]
pub struct PacketPair {
    p1: Packet,
    p2: Packet,
}

impl PacketPair {
    pub fn in_order(&self) -> bool {
        true
    }
}

impl fmt::Display for PacketPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1:{} p2:{}", &self.p1, &self.p2)
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
}
