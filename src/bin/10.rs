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

pub fn build_register_instructions(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|line| {
            if line.starts_with("addx ") {
                vec![0, line.replace("addx ", "").parse::<i32>().unwrap()] // takes 2 cycles to complete
            } else {
                vec![0] // noop takes 1 cycle but doesn't change register
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let register_instructions = build_register_instructions(input);

    let mut signal_strength = 0i32;
    let mut register = 1i32;
    for (idx, x) in register_instructions.iter().enumerate() {
        // adjust for 0-based indexing
        if (idx + 1) % 20 == 0 && idx > 0 {
            if idx + 1 == 20 || ((idx + 1) / 20) % 2 == 1 {
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
