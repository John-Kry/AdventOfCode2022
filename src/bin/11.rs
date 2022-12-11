use crate::Sign::{Minus, Multiply, Plus};
use crate::Value::{New, Old};
use std::collections::VecDeque;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let monkey_strings = input.split("\n\n").collect::<Vec<&str>>();
    let mut monkies: Vec<Monkey> = vec![];

    dbg!(&monkey_strings);

    let mut itemss: Vec<VecDeque<u32>> = vec![];
    for monkey in monkey_strings {
        let lines = monkey.lines().map(|s| s.trim()).collect::<Vec<&str>>();
        dbg!(&lines);
        let items = lines
            .get(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let operation = lines.get(2).unwrap();
        let split = operation.split(' ').collect::<Vec<&str>>();
        let math = Math {
            equals: split.get(1).unwrap().parse().unwrap(),
            sign: split.get(4).unwrap().parse().unwrap(),
            left: split.get(3).unwrap().to_string(),
            right: split.get(5).unwrap().to_string(),
        };

        let monkey = Monkey {
            operation: math,
            divisible_by: lines
                .get(3)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            target_true: lines
                .get(4)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            target_false: lines
                .get(5)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        };

        itemss.push(VecDeque::from(items));
        monkies.push(monkey);
    }
    let mut inspects: Vec<u32> = vec![0;monkies.len()];
    // one round
    for i in 0..20 {
        for (j, monkey) in monkies.iter_mut().enumerate() {
            while itemss.get(j).unwrap().len() > 0 {
                let mut worry_level = itemss.get_mut(j).unwrap().pop_front().unwrap();
                let math = &monkey.operation;
                let left_amount = math.left.parse::<u32>().unwrap_or(worry_level);

                let right_amount = math.right.parse::<u32>().unwrap_or(worry_level);
                match math.sign {
                    Plus => worry_level = left_amount + right_amount,
                    Minus => worry_level = left_amount - right_amount,
                    Multiply => worry_level = left_amount * right_amount,
                }
                worry_level = worry_level / 3;
                if worry_level % monkey.divisible_by == 0 {
                    itemss
                        .get_mut(monkey.target_true as usize)
                        .unwrap()
                        .push_back(worry_level);
                } else {
                    itemss
                        .get_mut(monkey.target_false as usize)
                        .unwrap()
                        .push_back(worry_level);
                }
                let new = inspects.get(j).unwrap_or(&0) + 1;
                inspects[j] = new;
                dbg!(worry_level);
            }
        }
    }
    dbg!(itemss);
    inspects.sort();
    inspects.reverse();
    dbg!(&inspects);
    Some(inspects[0] * inspects[1])
}

pub fn part_two(input: &str) -> Option<u32> {
    let monkey_strings = input.split("\n\n").collect::<Vec<&str>>();
    let mut monkies: Vec<Monkey> = vec![];

    dbg!(&monkey_strings);

    let mut itemss: Vec<VecDeque<u32>> = vec![];
    for monkey in monkey_strings {
        let lines = monkey.lines().map(|s| s.trim()).collect::<Vec<&str>>();
        dbg!(&lines);
        let items = lines
            .get(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let operation = lines.get(2).unwrap();
        let split = operation.split(' ').collect::<Vec<&str>>();
        let math = Math {
            equals: split.get(1).unwrap().parse().unwrap(),
            sign: split.get(4).unwrap().parse().unwrap(),
            left: split.get(3).unwrap().to_string(),
            right: split.get(5).unwrap().to_string(),
        };

        let monkey = Monkey {
            operation: math,
            divisible_by: lines
                .get(3)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            target_true: lines
                .get(4)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            target_false: lines
                .get(5)
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        };

        itemss.push(VecDeque::from(items));
        monkies.push(monkey);
    }
    let mut inspects: Vec<u32> = vec![0;monkies.len()];
    // one round
    for i in 0..20 {
        for (j, monkey) in monkies.iter_mut().enumerate() {
            while itemss.get(j).unwrap().len() > 0 {
                let mut worry_level = itemss.get_mut(j).unwrap().pop_front().unwrap();
                let math = &monkey.operation;
                let left_amount = math.left.parse::<u32>().unwrap_or(worry_level);

                let right_amount = math.right.parse::<u32>().unwrap_or(worry_level);
                match math.sign {
                    Plus => worry_level = left_amount + right_amount,
                    Minus => worry_level = left_amount - right_amount,
                    Multiply => worry_level = left_amount * right_amount,
                }
                worry_level = worry_level / 3;
                if worry_level % monkey.divisible_by == 0 {
                    itemss
                        .get_mut(monkey.target_true as usize)
                        .unwrap()
                        .push_back(worry_level);
                } else {
                    itemss
                        .get_mut(monkey.target_false as usize)
                        .unwrap()
                        .push_back(worry_level);
                }
                let new = inspects.get(j).unwrap_or(&0) + 1;
                inspects[j] = new;
                dbg!(worry_level);
            }
        }
    }
    dbg!(itemss);
    inspects.sort();
    inspects.reverse();
    dbg!(&inspects);
    Some(inspects[0] * inspects[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Monkey {
    operation: Math,
    divisible_by: u32,
    target_true: u32,
    target_false: u32,
}

struct Math {
    equals: Value,
    sign: Sign,
    left: String,
    right: String,
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
        dbg!(s);
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
        assert_eq!(part_two(&input), None);
    }
}
