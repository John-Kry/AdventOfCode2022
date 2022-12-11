use crate::Part::One;
use crate::Part::Two;
use crate::Sign::{Minus, Multiply, Plus};
use crate::Value::{New, Old};
use std::collections::VecDeque;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 20, One)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 10_000, Two)
}

fn solve(input: &str, iterations: usize, part: Part) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = vec![];

    let mut held_items_per_monkey: Vec<VecDeque<u64>> = vec![];
    for monkey in input.split("\n\n") {
        let lines = monkey.lines().map(|s| s.trim()).collect::<Vec<&str>>();
        let held_items = held_items(&lines);

        let operation = lines.get(2).unwrap();
        let math = Math::new(operation);
        let monkey = Monkey::new(lines, math);

        held_items_per_monkey.push(VecDeque::from(held_items));
        monkeys.push(monkey);
    }

    let mut modulo = 1;
    for monkey in &monkeys {
        modulo *= monkey.divisible_by
    }

    let mut inspects: Vec<u64> = vec![0; monkeys.len()];
    // one round
    for _ in 0..iterations {
        for (j, monkey) in monkeys.iter_mut().enumerate() {
            while held_items_per_monkey.get(j).unwrap().len() > 0 {
                let mut worry_level = held_items_per_monkey
                    .get_mut(j)
                    .unwrap()
                    .pop_front()
                    .unwrap();
                let math = &monkey.operation;
                let left_amount = math.left.parse::<u64>().unwrap_or(worry_level);
                let right_amount = math.right.parse::<u64>().unwrap_or(worry_level);

                match math.sign {
                    Plus => worry_level = left_amount + right_amount,
                    Minus => worry_level = left_amount - right_amount,
                    Multiply => worry_level = left_amount * right_amount,
                }
                match part {
                    One => {
                        worry_level = worry_level / 3;
                    }
                    Two => {
                        worry_level = worry_level % modulo;
                    }
                }
                if worry_level % monkey.divisible_by == 0 {
                    held_items_per_monkey
                        .get_mut(monkey.target_true)
                        .unwrap()
                        .push_back(worry_level);
                } else {
                    held_items_per_monkey
                        .get_mut(monkey.target_false)
                        .unwrap()
                        .push_back(worry_level);
                }
                inspects[j] = inspects.get(j).unwrap_or(&0) + 1;
            }
        }
    }
    inspects.sort();
    inspects.reverse();
    Some(inspects[0] * inspects[1])
}

fn held_items(lines: &Vec<&str>) -> Vec<u64> {
    lines
        .get(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(", ")
        .map(|item| item.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

enum Part {
    One,
    Two,
}

struct Monkey {
    operation: Math,
    divisible_by: u64,
    target_true: usize,
    target_false: usize,
}

impl Monkey {
    fn new(lines: Vec<&str>, math: Math) -> Self {
        Self {
            operation: math,
            divisible_by: lines
                .get(3)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            target_true: lines
                .get(4)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            target_false: lines
                .get(5)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        }
    }
}

struct Math {
    sign: Sign,
    left: String,
    right: String,
}

impl Math {
    fn new(operation: &&str) -> Math {
        let split = operation.split(' ').collect::<Vec<&str>>();
        Self {
            sign: split.get(4).unwrap().parse().unwrap(),
            left: split.get(3).unwrap().to_string(),
            right: split.get(5).unwrap().to_string(),
        }
    }
}

enum Value {
    New,
    Old,
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new" {
            Ok(New)
        } else {
            Ok(Old)
        }
    }
}

enum Sign {
    Plus,
    Minus,
    Multiply,
}

impl FromStr for Sign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(match s {
            "+" => Plus,
            "-" => Minus,
            "*" => Multiply,
            _ => {
                unreachable!()
            }
        });
    }
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
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
