use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::usize;

pub fn part_one(input: &str) -> Option<String> {
    let (setup, moves) = input.split_once("\n\n").unwrap();
    let number_of_stacks: usize = setup
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();
    let mut stacks = Stacks::new(number_of_stacks, setup);

    moves.lines().for_each(|line| {
        let dir: Direction = line.parse().unwrap();
        stacks.execute_directions(dir);
    });
    Some(String::from(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let (setup, moves) = input.split_once("\n\n").unwrap();
    let number_of_stacks: usize = setup
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();

    let mut stacks = Stacks::new(number_of_stacks, setup);

    moves.lines().for_each(|line| {
        let dir: Direction = line.parse().unwrap();
        stacks.execute_directions_at_once(dir);
    });
    Some(String::from(stacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Direction {
    amount: u32,
    from: usize,
    to: usize,
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();
        let amount = split[1].parse::<u32>()?;
        let from = split[3].parse::<usize>()? - 1;
        let to = split[5].parse::<usize>()? - 1;
        Ok(Self { amount, from, to })
    }
}

impl From<Stacks> for String {
    fn from(stacks: Stacks) -> Self {
        String::from_iter(stacks.stacks.iter().filter_map(|v| v.last()))
    }
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new(number_of_stacks: usize, setup: &str) -> Self {
        let mut stacks: Vec<Vec<char>> = Vec::new();

        for _ in 0..number_of_stacks {
            stacks.push(Vec::new());
        }

        let mut map: HashMap<usize, usize> = HashMap::new();
        map.insert(1, 0);
        for i in 0..=9 {
            map.insert(1 + 4 * i, i);
        }

        setup.lines().rev().skip(1).for_each(|line| {
            line.chars().enumerate().for_each(|(pos, c)| {
                if !c.is_whitespace() && c != '[' && c != ']' {
                    let stack_num = map.get(&pos).copied().unwrap();
                    stacks[stack_num].push(c);
                }
            })
        });

        Self { stacks }
    }

    fn move_char(&mut self, from: usize, to: usize) {
        let prev = self.stacks[from].pop().unwrap();
        self.stacks[to].push(prev);
    }
    fn execute_directions(&mut self, dir: Direction) {
        (0..dir.amount).for_each(|_| {
            self.move_char(dir.from, dir.to);
        });
    }
    fn execute_directions_at_once(&mut self, dir: Direction) {
        let items: Vec<char> = (0..dir.amount)
            .map(|_| self.stacks[dir.from].pop().unwrap())
            .collect();

        items.iter().rev().for_each(|item| {
            self.stacks[dir.to].push(*item);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
