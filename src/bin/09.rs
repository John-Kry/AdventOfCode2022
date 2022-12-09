use crate::Dir::{D, L, R, U};
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut p = Players {
        head: Pos { x: 0, y: 0 },
        tail: Pos { x: 0, y: 0 },
    };
    let mut seen: HashMap<Pos, bool> = HashMap::new();
    let lines = input.lines();
    lines.for_each(|line| {
        let splits = line.split_once(' ').unwrap();
        let dir = Dir::from(splits.0.chars().next().unwrap());
        let amount = splits.1.parse::<u32>().unwrap();

        for _ in 0..amount {
            match dir {
                R => p.head.x += 1,
                L => p.head.x -= 1,
                U => p.head.y += 1,
                D => p.head.y -= 1,
            }
            p.catch_up();

            seen.insert(
                Pos {
                    x: p.tail.x,
                    y: p.tail.y,
                },
                true,
            );
        }
    });
    Some(seen.iter().count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut container = Container::new();
    let mut seen: Vec<HashSet<Pos>> = vec![];
    const TAIL_LENGTH: usize = 9;
    for _ in 0..10 {
        seen.push(HashSet::default());
    }
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
            for i in 1..=TAIL_LENGTH {
                let prev = container.items.get(i - 1).unwrap().clone();
                let curr = container.items.get_mut(i).unwrap();
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
                if i == TAIL_LENGTH {
                    seen.get_mut(i).unwrap().insert(curr.clone());
                }
            }
        }
    });
    Some(seen.get(TAIL_LENGTH as usize).unwrap().len() as u32)
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

#[derive(Debug)]
struct Players {
    head: Pos,
    tail: Pos,
}

impl Default for Players {
    fn default() -> Self {
        Players {
            head: Pos { x: 0, y: 0 },
            tail: Pos { x: 0, y: 0 },
        }
    }
}

impl Players {
    fn is_on_same_line(&self) -> bool {
        return self.head.x == self.tail.x || self.head.y == self.tail.y;
    }

    fn catch_up(&mut self) {
        if (self.head.x - self.tail.x).abs() <= 1 && (self.head.y - self.tail.y).abs() <= 1 {
            return;
        }

        if self.is_on_same_line() {
            if self.head.x == self.tail.x {
                if self.head.y > self.tail.y {
                    self.tail.y += 1;
                } else {
                    self.tail.y -= 1;
                }
            } else {
                if self.head.x > self.tail.x {
                    self.tail.x += 1;
                } else {
                    self.tail.x -= 1;
                }
            }
        } else {
            //up
            if self.head.y > self.tail.y {
                // right
                if self.head.x > self.tail.x {
                    self.tail.y += 1;
                    self.tail.x += 1;
                } else {
                    // left
                    self.tail.y += 1;
                    self.tail.x -= 1;
                }
                // down
            } else {
                // right
                if self.head.x > self.tail.x {
                    self.tail.y -= 1;
                    self.tail.x += 1;
                } else {
                    // left
                    self.tail.y -= 1;
                    self.tail.x -= 1;
                }
            }
        }
    }
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

    #[test]
    fn catch_up_works() {
        let mut p = Players::default();
        p.head.x = 0;
        p.head.y = 0;
        p.tail.x = 0;
        p.tail.y = 0;
        p.catch_up();

        p.head.y += 1;
        p.catch_up();
    }
}
