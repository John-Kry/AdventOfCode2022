use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::Lines;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut set = parse(lines);
    let abyss_y = set.iter().map(|s| s.y).max().unwrap();
    loop {
        let mut sand = Pos {
            x: 500,
            y: 0,
            kind: Kind::Sand,
        };
        loop {
            let mut check = sand.clone();
            check.y += 1;
            let mut left = sand.clone();
            left.x -= 1;
            left.y += 1;
            let mut right = sand.clone();
            right.x += 1;
            right.y += 1;
            if !set.contains(&check) {
                sand.y += 1;
            } else if !set.contains(&left) {
                sand.y += 1;
                sand.x -= 1;
            } else if !set.contains(&right) {
                sand.y += 1;
                sand.x += 1
            } else {
                set.insert(sand);
                break;
            }
            if sand.y >= abyss_y {
                return Some(set.iter().filter(|s| s.kind == Kind::Sand).count() as u32);
            }
        }
    }
}

fn print(set: &HashSet<Pos>) {
    for y in 0..=9 {
        for x in 494..=503 {
            if let Some(pos) = set.get(&Pos {
                x,
                y,
                kind: Kind::Wall,
            }) {
                print!("{}", pos.kind.to_char().to_string());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn dir(prev: Option<Pos>, new_pos: Pos) -> Option<(i32, i32)> {
    if prev.is_some() {
        if prev.unwrap().x == new_pos.x {
            if prev.unwrap().y < new_pos.y {
                return Some((0, 1));
            } else {
                return Some((0, -1));
            }
        } else {
            if prev.unwrap().x < new_pos.x {
                return Some((1, 0));
            } else {
                return Some((-1, 0));
            }
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut set = parse(lines);
    let abyss_y = set.iter().map(|s| s.y).max().unwrap();
    for i in -1000..1000 {
        set.insert(Pos {
            x: i,
            y: abyss_y + 2,
            kind: Kind::Wall,
        });
    }
    let origin = Pos {
        x: 500,
        y: 0,
        kind: Kind::Sand,
    };
    loop {
        if set.contains(&origin) {
            return Some(set.iter().filter(|s| s.kind == Kind::Sand).count() as u32);
        }
        let mut sand = origin.clone();
        loop {
            let mut left = sand.clone();
            left.x -= 1;
            left.y += 1;
            let mut right = sand.clone();
            right.x += 1;
            right.y += 1;
            if !set.contains(&Pos {
                x: sand.x,
                y: sand.y + 1,
                kind: Kind::Wall,
            }) {
                sand.y += 1;
            } else if !set.contains(&left) {
                sand.y += 1;
                sand.x -= 1;
            } else if !set.contains(&right) {
                sand.y += 1;
                sand.x += 1
            } else {
                set.insert(sand);
                break;
            }
        }
    }
}

fn parse(lines: Lines) -> HashSet<Pos> {
    let mut set: HashSet<Pos> = HashSet::new();
    lines.for_each(|line| {
        let mut prev: Option<Pos> = None;
        line.split(' ').step_by(2).into_iter().for_each(|c| {
            let (x_str, y_str) = c.split_once(',').unwrap();
            let x: i32 = x_str.parse().unwrap();
            let y: i32 = y_str.parse().unwrap();
            let new_pos = Pos {
                x: x,
                y: y,
                kind: Kind::Wall,
            };
            if let Some(dir) = dir(prev, new_pos) {
                let mut curr = prev.unwrap().clone();
                while curr != new_pos {
                    set.insert(curr.clone());
                    curr.x += dir.0;
                    curr.y += dir.1;
                }
            }
            set.insert(new_pos);
            prev = Some(new_pos.clone());
        })
    });
    set
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    kind: Kind,
}
impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        state.finish();
    }
}

#[derive(PartialEq, Copy, Clone, Eq, Debug)]
enum Kind {
    Wall,
    Sand,
    Open,
}

impl Kind {
    fn to_char(&self) -> &str {
        match self {
            Kind::Wall => "#",
            Kind::Sand => "o",
            Kind::Open => ".",
        }
    }
}

impl PartialEq<Self> for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pos {}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn poggers() {
        let mut set: HashSet<Pos> = HashSet::new();
        set.insert(Pos {
            x: 0,
            y: 0,
            kind: Kind::Wall,
        });
        set.insert(Pos {
            x: 0,
            y: 0,
            kind: Kind::Sand,
        });

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
