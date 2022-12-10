use ansi_term::Colour::Green;

pub struct Display {
    pixels: Vec<String>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: vec![String::from(" "); 240],
        }
    }
    pub fn show(&self) {
        for (idx, s) in self.pixels.iter().enumerate() {
            print!("{}", s);
            if (idx + 1) % 40 == 0 {
                println!("");
            }
        }
    }
}

// bet I could do this functionally...
pub fn build_register_instructions(input: &str) -> Vec<i32> {
    let mut r = vec![];
    for instruction in input.lines() {
        if instruction.starts_with("addx ") {
            let val = instruction.replace("addx ", "").parse::<i32>().unwrap();
            // take 2 cycles to complete
            r.push(0);
            r.push(val);
        } else if instruction.starts_with("noop") {
            // takes 1 cycle to complete
            r.push(0);
        }
    }
    r
}

pub fn part_one(input: &str) -> Option<i32> {
    let register_instructions = build_register_instructions(input);

    let mut signal_strength = 0i32;
    let mut register = 1i32;
    for (idx, x) in register_instructions.iter().enumerate() {
        if (idx + 1) % 20 == 0 && idx > 0 {
            if idx + 1 == 20 || ((idx + 1) / 20) % 2 == 1 {
                // println!("x at {:?} is {:?}", &idx + 1, &register);
                signal_strength += register * (idx + 1) as i32;
            }
        }
        register += x;
    }
    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let register_instructions = build_register_instructions(input);

    let mut d = Display::new();

    let mut register = 1i32;
    for (idx, x) in register_instructions.iter().enumerate() {
        if (idx % 40) as i32 >= register - 1 && (idx % 40) as i32 <= register + 1 {
            d.pixels[idx] = Green.bold().paint("#").to_string();
        }
        register += x;
    }

    d.show();
    Some(String::from("read the letters above ^^"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from("read the letters above ^^"))
        );
    }
}
