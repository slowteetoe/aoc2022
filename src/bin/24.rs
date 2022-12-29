use multimap::MultiMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// each snowflake has a position and a direction
pub enum Entity {
    WALL,
    UPWARD,
    DOWNWARD,
    RIGHTWARD,
    LEFTWARD,
    // EXPEDITION(usize, usize),
}

pub enum Outcome {
    SUCCESS,
    DEAD,
}

#[derive(Debug)]
pub struct Grid {
    x: usize,
    y: usize,
    expedition: (usize, usize),
    data: MultiMap<(usize, usize), Entity>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let (x, y, data) = parse(input);
        Self {
            x,
            y,
            expedition: (1, 0),
            data,
        }
    }

    pub fn tick(&mut self) -> Option<Outcome> {
        // avoiding 0s and grid.len() because they're all walls (or expedition/exits that will be handled separately)
        // go through and calculate all the new positions
        let mut new_pos = vec![];
        for y in 1..self.y {
            for x in 1..self.x {
                // check every cell and see if there's anything that needs to be moved
                let entities = self.data.get_vec(&(x, y));
                if entities.is_none() {
                    continue;
                }
                entities.unwrap().iter().for_each(|e| {
                    let this_move = match e {
                        Entity::UPWARD => {
                            if y == 1 {
                                // this gets spawned at the bot
                                ((x, self.y - 2), Entity::UPWARD)
                            } else {
                                ((x, y - 1), Entity::UPWARD)
                            }
                        }
                        Entity::DOWNWARD => {
                            if y == self.y - 2 {
                                ((x, 1), Entity::DOWNWARD)
                            } else {
                                ((x, y + 1), Entity::DOWNWARD)
                            }
                        }
                        Entity::RIGHTWARD => {
                            if x == self.x - 2 {
                                ((1, y), Entity::RIGHTWARD)
                            } else {
                                ((x + 1, y), Entity::RIGHTWARD)
                            }
                        }
                        Entity::LEFTWARD => {
                            if x == 1 {
                                ((self.x - 2, y), Entity::LEFTWARD)
                            } else {
                                ((x - 1, y), Entity::LEFTWARD)
                            }
                        }
                        // this really isn't necessary, I should just figure out how to safely ignore
                        // Entity::EXPEDITION(_, _) => ((x, y), Entity::EXPEDITION(x, y)),
                        Entity::WALL => ((x, y), Entity::WALL),
                    };
                    new_pos.push(this_move);
                });
                // clear the existing entities
                self.data.remove(&(x, y));
            }
        }
        for (point, entity) in new_pos.iter() {
            self.data.insert(*point, entity.clone());
        }
        // now it's the expedition's turn to move, we'll bias down and right
        // depending on how twisted this puzzle is, we might have to keep move state and backtrack when the expedition dies
        // but for now, let's assume there's always a safe move and that we don't need to look ahead
        let (x, y) = self.expedition;
        if self.data.get(&(x + 1, y)).is_none() {
            // println!("expedition moves right");
            self.expedition = (x + 1, y);
        } else if self.data.get(&(x, y + 1)).is_none() {
            // println!("expedition moves down");
            self.expedition = (x, y + 1);
        } else if y >= 1 && self.data.get(&(x, y - 1)).is_none() {
            // you can go back into the starting point, I guess, but not any further
            // println!("expedition moves up");
            self.expedition = (x, y - 1);
        } else if self.data.get(&(x - 1, y)).is_none() {
            // println!("expedition moves left");
            self.expedition = (x - 1, y);
        } else if self.data.get(&(x, y)).is_none()
        // || *self.data.get(&(x, y)).unwrap() == Entity::EXPEDITION(x, y)
        {
            // println!("wait")
        } else {
            // println!("YOU DEAD");
            return Some(Outcome::DEAD);
        }
        if self.expedition == (self.x - 2, self.y - 1) {
            println!("You navigated the perilious snows safely!");
            Some(Outcome::SUCCESS)
        } else {
            None
        }
    }

    pub fn display(&self) {
        let mut g = String::from("");
        for y in 0..self.y {
            for x in 0..self.x {
                if self.expedition == (x, y) {
                    g.push('E');
                } else if self.data.contains_key(&(x, y)) {
                    let entities = self.data.get_vec(&(x, y)).unwrap();
                    if entities.len() > 1 {
                        g.push(char::from_digit(entities.len() as u32, 10u32).unwrap());
                    } else {
                        g.push(match entities.get(0).unwrap() {
                            // Entity::EXPEDITION(_, _) => 'E',
                            Entity::WALL => '#',
                            Entity::DOWNWARD => 'v',
                            Entity::UPWARD => '^',
                            Entity::LEFTWARD => '<',
                            Entity::RIGHTWARD => '>',
                            // _ => unreachable!(),
                        });
                    }
                } else {
                    g.push('.');
                }
            }
            g.push('\n');
        }
        println!("{g}");
    }
}

pub fn parse(input: &str) -> (usize, usize, MultiMap<(usize, usize), Entity>) {
    let mut m = MultiMap::new();

    let mut max_y = 0usize;
    let mut max_x = 0usize;
    input.lines().enumerate().for_each(|(y, line)| {
        max_y = y;
        line.chars().enumerate().for_each(|(x, ch)| {
            match ch {
                '#' => m.insert((x, y), Entity::WALL),
                '<' => m.insert((x, y), Entity::LEFTWARD),
                '>' => m.insert((x, y), Entity::RIGHTWARD),
                'v' => m.insert((x, y), Entity::DOWNWARD),
                '^' => m.insert((x, y), Entity::UPWARD),
                _ => (),
            };
            max_x = x;
        });
    });
    // correct for zero-based
    (max_x + 1, max_y + 1, m)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);
    // grid.display();
    for round in 0..256 {
        let status = grid.tick();
        grid.display();
        if status.is_some() {
            match status.unwrap() {
                Outcome::SUCCESS => return Some(round + 1),
                Outcome::DEAD => panic!("YOU DEAD!!!!!"),
            }
        }
    }
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
