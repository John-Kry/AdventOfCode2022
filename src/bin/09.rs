use crate::Dir::{D, L, R, U};
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(&input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(&input, 9))
}

fn solve(input: &str, tail_length: usize) -> u32 {
    let mut container = Container::new();
    let mut seen: HashSet<Pos> = HashSet::default();
    let lines = input.lines();
    lines.for_each(|line| {
        let splits = line.split_once(' ').unwrap();
        let dir = Dir::from(splits.0.chars().next().unwrap());
        let amount = splits.1.parse::<u32>().unwrap();
        for _ in 0..amount {
            let init = container.items.get_mut(0).unwrap();
            match dir {
                R => init.x += 1,
                L => init.x -= 1,
                U => init.y += 1,
                D => init.y -= 1,
            }
            for i in 1..=tail_length {
                let prev = container.items.get(i - 1).unwrap().clone();
                let curr = container.items.get_mut(i).unwrap();
                mutate_curr_pos(curr, &prev);
                if i == tail_length {
                    seen.insert(curr.clone());
                }
            }
        }
    });

    seen.len() as u32
}
fn mutate_curr_pos(curr: &mut Pos, prev: &Pos) {
    if (prev.x - curr.x).abs() > 1 || (prev.y - curr.y).abs() > 1 {
        // If on the same line
        if prev.x == curr.x || prev.y == curr.y {
            if prev.x == curr.x {
                // X is the same, move Y position
                if prev.y > curr.y {
                    curr.y += 1;
                } else {
                    curr.y -= 1;
                }
            } else {
                // Y is the same, move X position
                if prev.x > curr.x {
                    curr.x += 1;
                } else {
                    curr.x -= 1;
                }
            }
        }
        // If on different line
        else {
            //up
            if prev.y > curr.y {
                if prev.x > curr.x {
                    // right diagonal
                    curr.y += 1;
                    curr.x += 1;
                } else {
                    // left diagonal
                    curr.y += 1;
                    curr.x -= 1;
                }
            }
            // down
            else {
                if prev.x > curr.x {
                    // right diagonal
                    curr.y -= 1;
                    curr.x += 1;
                } else {
                    // left diagonal
                    curr.y -= 1;
                    curr.x -= 1;
                }
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Container {
    items: Vec<Pos>,
}

impl Container {
    fn new() -> Self {
        let mut items: Vec<Pos> = vec![];
        for _ in 0..10 {
            items.push(Pos::default())
        }
        Self { items }
    }
}

#[derive(Debug)]
pub enum Dir {
    R,
    L,
    U,
    D,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'R' => R,
            'L' => L,
            'U' => U,
            'D' => D,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Default, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
