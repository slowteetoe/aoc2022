use linked_hash_set::LinkedHashSet;

struct Cornelius {
    pos: (usize, usize),
    route: LinkedHashSet<(usize, usize)>,
    map: TopographicalMap,
    food: usize, // just for fun, and so he doesn't wander forever
}

pub struct TopographicalMap {
    terrain: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    max_x: usize,
    max_y: usize,
}

impl TopographicalMap {
    pub fn new(terrain: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Self {
        let max_x = terrain.get(0).unwrap().len();
        let max_y = terrain.len();
        Self {
            terrain: terrain,
            start: start,
            end: end,
            max_x,
            max_y,
        }
    }

    pub fn height_of(&self, x: usize, y: usize) -> usize {
        self.terrain.get(y).unwrap().get(x).unwrap().clone() as usize
    }
}

impl Cornelius {
    pub fn new(map: TopographicalMap) -> Self {
        let food = map.max_x * map.max_y;
        Self {
            pos: map.start,
            route: LinkedHashSet::new(),
            map,
            food,
        }
    }

    fn navigate(&mut self) -> Option<usize> {
        while self.pos != self.map.end && self.food > 0 {
            self.take_step();
        }
        if self.pos == self.map.end {
            println!("*** SUCCESS in {} moves ***", self.route.len());
            Some(self.route.len())
        } else {
            println!("💀💀 You wandered aimlessly and died of starvation 💀💀");
            None
        }
    }

    // oh this is begging for refactoring
    fn take_step(&mut self) {
        // pick a direction we can go, probably need to deal with avoiding loops (aside from starving)
        // go right
        if self.pos.0 < self.map.max_x
            && self.map.height_of(self.pos.0 + 1, self.pos.1)
                <= self.map.height_of(self.pos.0, self.pos.1) + 1
            && !self.route.contains(&(self.pos.0 + 1, self.pos.1))
        {
            self.pos.0 += 1;
            println!("I go right! {:?}", self.pos)
        } else if self.pos.0 > 0
            && self.map.height_of(self.pos.0 - 1, self.pos.1)
                <= self.map.height_of(self.pos.0, self.pos.1) + 1
            && !self.route.contains(&(self.pos.0 - 1, self.pos.1))
        {
            self.pos.0 -= 1;
            println!("Left! {:?}", self.pos);
        } else if self.pos.1 < self.map.max_y
            && self.map.height_of(self.pos.0, self.pos.1 + 1)
                <= self.map.height_of(self.pos.0, self.pos.1) + 1
            && !self.route.contains(&(self.pos.0, self.pos.1 + 1))
        {
            self.pos.1 += 1;
            println!("Down {:?}", self.pos);
        } else if self.pos.1 > 0
            && self.map.height_of(self.pos.0, self.pos.1 - 1)
                <= self.map.height_of(self.pos.0, self.pos.1) + 1
            && !self.route.contains(&(self.pos.0, self.pos.1 - 1))
        {
            self.pos.1 -= 1;
            println!("Up {:?}", self.pos);
        } else {
            // if there aren't any, backtrack using route?
            unreachable!("you dun messsed up");
        }
        self.food -= 1;
        self.route.insert(self.pos.clone());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = build_topo(input);
    let mut explorer = Cornelius::new(map);
    explorer.navigate();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

// returns map, starting point and ending point
pub fn build_topo(input: &str) -> TopographicalMap {
    let mut terrain = vec![];
    let mut row = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for line in input.lines() {
        let mut this_row = vec![];
        let mut col = 0;
        for ch in line.chars() {
            if ch == 'S' {
                start = (col as usize, row as usize);
                this_row.push('a');
            } else if ch == 'E' {
                end = (col as usize, row as usize);
                this_row.push('z');
            } else {
                this_row.push(ch);
            }
            col += 1;
        }
        terrain.push(this_row);
        row += 1;
    }
    TopographicalMap::new(terrain, start, end)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
