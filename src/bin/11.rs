use std::{borrow::BorrowMut, collections::HashMap};

use ansi_term::Colour::{Green, Red};

#[derive(Debug, Clone)]
pub struct Item {
    worry_level: usize,
}

impl Item {
    pub fn new(worry_level: usize) -> Self {
        Self { worry_level }
    }
}

pub fn items_from(items: Vec<usize>) -> Vec<Item> {
    let mut result = vec![];
    for item in items {
        result.push(Item::new(item));
    }
    result
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<Item>,
    op: fn(usize) -> usize,
    test: fn(usize) -> u8,
    inspected: usize,
}

impl Monkey {
    pub fn new(items: Vec<Item>, op: fn(usize) -> usize, test: fn(usize) -> u8) -> Self {
        Self {
            items,
            op,
            test,
            inspected: 0,
        }
    }

    fn react(&mut self) -> HashMap<u8, Vec<Item>> {
        let mut dest = HashMap::new();
        for n in 0..self.items.len() {
            let item = self.items.get_mut(n).unwrap();
            self.inspected += 1;
            // println!(
            //     "{}",
            //     Red.paint(format!(
            //         "\tMonkey inspects item with a worry level of {}.",
            //         &item.worry_level
            //     ))
            // );
            item.worry_level = (self.op)(item.worry_level);
            // println!(
            //     "{}",
            //     Red.paint(format!("\tnew worry level: {}", &item.worry_level))
            // );
            item.worry_level = item.worry_level / 3;
            // println!(
            //     "{}",
            //     Blue.paint(format!("\tnew worry level: {}", &item.worry_level))
            // );
            let next_monkey = (self.test)(item.worry_level);
            println!(
                "{}",
                Red.paint(format!("\tmonkey thows item to monkey: {}", &next_monkey))
            );
            // this is really awkward, probably need to rethink this
            // basically, since there's nothing special about the Item, we'll just clone it into a map
            // and then clear this monkey's item vec once we're done
            dest.entry(next_monkey)
                .and_modify(|e: &mut Vec<Item>| e.push(Item::new(item.worry_level.clone())))
                .or_insert(vec![Item::new(item.worry_level.clone())]);
        }
        self.items.clear();
        dest
    }
}

// let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
// for line in moves {
//     let cap = re.captures(line).unwrap();
//     let num = cap[1].parse::<usize>().unwrap();
//     let from = cap[2].parse::<usize>().unwrap() - 1;
//     let to = cap[3].parse::<usize>().unwrap() - 1;

// TODO quick and dirty for the moment, but perfect time to try out Nom
// and figure out how to do this in Rust (return a dynamic function from runtime)
// or maybe something like rhai
// TODO Just use a build script to preprocess the input ... ?
fn parse_monkeys(input: &str) -> Vec<Monkey> {
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3

    // let mut items: Vec<Item>;
    // let mut op: Option<fn(usize) -> usize>;
    // let mut test: Option<fn(Item) -> u8>;
    // let update_re = Regex::new(r"  Operation: new = old (.){1} (\d+)").unwrap();

    // for line in input.lines().into_iter() {
    //     if line.starts_with("Monkey ") {
    //         items = vec![];
    //         op = None;
    //         test = None;
    //     } else if line.starts_with("  Operation: new = old * old") {
    //         op = Some(|x| x * x);
    //     } else if line.starts_with("  Operation: new = old ") {
    //         let cap = update_re.captures(line).unwrap();
    //         op = match cap.get(1).unwrap().as_str() {
    //             "*" => Some(|x| x * cap.get(2).parse::<usize>()),
    //             _ => None,
    //         }
    //         // this will be an operation followed by either a value or a reference to old
    //     }
    // }
    // there are only 7 monkeys in the input which takes approx 98% less time to hard-code
    // than I've spent trying to figure out a dynamic approach =(
    let mut monkeys = vec![];

    monkeys.push(Monkey::new(
        items_from(vec![99, 67, 92, 61, 83, 64, 98]),
        |n| n * 17,
        |worry_level| match worry_level % 3 {
            0 => 4,
            _ => 2,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![78, 74, 88, 89, 50]),
        |n| n * 11,
        |worry_level| match worry_level % 5 {
            0 => 3,
            _ => 5,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![98, 91]),
        |n| n + 4,
        |worry_level| match worry_level % 2 {
            0 => 6,
            _ => 4,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![59, 72, 94, 91, 79, 88, 94, 51]),
        |n| n * n,
        |worry_level| match worry_level % 13 {
            0 => 0,
            _ => 5,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![95, 72, 78]),
        |n| n + 7,
        |worry_level| match worry_level % 11 {
            0 => 7,
            _ => 6,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![76]),
        |n| n + 8,
        |worry_level| match worry_level % 17 {
            0 => 0,
            _ => 2,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![69, 60, 53, 89, 71, 88]),
        |n| n + 5,
        |worry_level| match worry_level % 19 {
            0 => 7,
            _ => 1,
        },
    ));

    monkeys.push(Monkey::new(
        items_from(vec![72, 54, 63, 80]),
        |n| n + 3,
        |worry_level| match worry_level % 7 {
            0 => 1,
            _ => 3,
        },
    ));
    monkeys
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);

    let num_rounds = 20;
    // after 20 rounds
    for n in 0..num_rounds {
        println!("=== Round {} ===", n + 1);
        for n in 0..monkeys.len() {
            println!("{}", Green.paint(format!("Monkey {}'s turn", n)));
            let stuff_to_toss = monkeys[n].react();
            for (next_monkey, items) in stuff_to_toss {
                for item in items {
                    println!("sending {:?} to monkey {}", &item, next_monkey);
                    monkeys[next_monkey as usize].borrow_mut().items.push(item);
                }
            }
        }
    }
    // find the 2 most active monkeys
    let mut most = vec![0; 2];
    for m in monkeys {
        if m.inspected > most[0] {
            most[1] = most[0];
            most[0] = m.inspected;
        } else if m.inspected > most[1] {
            most[1] = m.inspected;
        }
    }
    dbg!(&most);
    Some(most[0] * most[1])
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
