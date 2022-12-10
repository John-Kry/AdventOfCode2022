use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let mut cpu = Cpu { x: 1, cycle: 1 };
    let lines = input.lines();
    let mut sum_signal_strength = 0;
    lines.for_each(|line| {
        let instruction = line.parse::<Instruction>().unwrap();
        for _ in 0..instruction.cycle_length() {
            if cpu.cycle == 20 {
                sum_signal_strength += cpu.signal_strength();
            } else if cpu.cycle > 20 && ((cpu.cycle - 20) % 40) == 0 {
                sum_signal_strength += cpu.signal_strength();
            }
            cpu.cycle += 1;
        }
        cpu.mutate_x(instruction, line);
    });
    Some(sum_signal_strength as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cpu = Cpu { x: 1, cycle: 1 };
    let lines = input.lines();
    let mut crt_pos: usize = 0;
    let mut screen: Vec<bool> = Vec::with_capacity(240);
    lines.for_each(|line| {
        let instruction = line.parse::<Instruction>().unwrap();
        for _ in 0..instruction.cycle_length() {
            if (cpu.x - crt_pos as i32).abs() <= 1 {
                screen.push(true);
            } else {
                screen.push(false);
            }

            crt_pos += 1;
            if crt_pos >= 40 {
                crt_pos = 0
            }
            cpu.cycle += 1;
        }
        cpu.mutate_x(instruction, line);
    });

    for (k, val) in screen.iter().enumerate() {
        print!("{}", if *val { "█" } else { " " });
        if (k + 1) % 40 == 0 {
            println!();
        }
    }
    // Some(0), because the answer is printed above
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Cpu {
    x: i32,
    cycle: u32,
}

impl Cpu {
    fn signal_strength(&self) -> i32 {
        self.x * self.cycle as i32
    }
    fn mutate_x(&mut self, ins: Instruction, line: &str) {
        match ins {
            Instruction::Add => {
                let split = line.split_once(' ').unwrap();
                self.x += split.1.parse::<i32>().unwrap();
            }
            Instruction::Noop => {}
        }
    }
}

impl Instruction {
    fn cycle_length(&self) -> u32 {
        match self {
            Instruction::Add => 2,
            Instruction::Noop => 1,
        }
    }
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("add") {
            return Ok(Instruction::Add);
        } else {
            return Ok(Instruction::Noop);
        }
    }
}

enum Instruction {
    Add,
    Noop,
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
        assert_eq!(part_two(&input), Some(0));
    }
}
