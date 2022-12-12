use crate::Part::{One, Two};
use std::collections::VecDeque;
use std::convert::identity;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);

    let mut queue: VecDeque<(Pos, u32)> = VecDeque::new();
    let data = grid
        .data
        .get_mut(grid.start.x as usize)
        .unwrap()
        .get_mut(grid.start.y as usize)
        .unwrap();
    data.1 = true;
    queue.push_back((grid.start, 0));

    grid.shortest_path(queue, One)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);
    // Set starting to end, and search for 'a'
    grid.start = grid.end.clone();

    let mut queue: VecDeque<(Pos, u32)> = VecDeque::new();
    let data = grid
        .data
        .get_mut(grid.start.x as usize)
        .unwrap()
        .get_mut(grid.start.y as usize)
        .unwrap();
    data.1 = true;

    queue.push_back((grid.start, 0));
    grid.shortest_path(queue, Two)
}

fn is_valid(part: Part, curr: (u32, bool), next: (u32, bool)) -> bool {
    return match part {
        One => (next.0 as i32 - curr.0 as i32) <= 1,
        Two => (curr.0 as i32 - next.0 as i32) <= 1,
    };
}

#[derive(Clone, Copy)]
enum Part {
    One,
    Two,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

struct Grid {
    data: Vec<Vec<(u32, bool)>>,
    start: Pos,
    end: Pos,
    x_len: usize,
    y_len: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {}

impl Grid {
    fn shortest_path(&mut self, mut queue: VecDeque<(Pos, u32)>, part: Part) -> Option<u32> {
        while queue.len() > 0 {
            let (v, count) = queue.pop_front().unwrap();
            let curr = self.get(v.x, v.y).unwrap();

            match part {
                One => {
                    if v == self.end {
                        return Some(count);
                    }
                }
                Two => {
                    if curr.0 == ('a' as u32) {
                        return Some(count);
                    }
                }
            }

            let left = self.new_pos(v.x as i32 - 1, v.y as i32);
            let right = self.new_pos(v.x as i32 + 1, v.y as i32);
            let up = self.new_pos(v.x as i32, v.y as i32 + 1);
            let down = self.new_pos(v.x as i32, v.y as i32 - 1);

            let mut dirs: Vec<Option<Pos>> = Vec::new();
            dirs.push(left);
            dirs.push(right);
            dirs.push(up);
            dirs.push(down);

            dirs.iter().filter_map(|x| x.as_ref()).for_each(|d| {
                if let Some(next) = self.get(d.x, d.y) {
                    if next.1 == false && is_valid(part, curr, next) {
                        self.set_visited(d.x, d.y);
                        queue.push_back((d.clone(), count + 1));
                    }
                }
            })
        }
        None
    }

    fn new_pos(&self, x: i32, y: i32) -> Option<Pos> {
        if x < 0 || y < 0 || x as usize >= self.x_len || y as usize >= self.y_len {
            return None;
        }
        Some(Pos {
            x: x as usize,
            y: y as usize,
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<(u32, bool)> {
        if let Some(poss) = self.data.get(x as usize) {
            return poss.get(y).copied();
        }
        None
    }

    fn set_visited(&mut self, x: usize, y: usize) {
        self.data.get_mut(x).unwrap().get_mut(y).unwrap().1 = true
    }

    fn new(input: &str) -> Self {
        let lines = input.lines();
        let x_len = lines.clone().next().unwrap().len();
        let y_len = lines.clone().count();

        let mut data: Vec<Vec<(u32, bool)>> = vec![];
        let mut start = Pos { x: 0, y: 0 };
        let mut end = Pos { x: 0, y: 0 };
        for curr_x in 0..x_len {
            data.push(vec![]);
            let curr = data.get_mut(curr_x).unwrap();
            lines.clone().for_each(|line| {
                let c = line.as_bytes().get(curr_x).unwrap().clone() as char;
                if c == 'S' {
                    start.x = curr_x;
                    start.y = curr.len();
                    curr.push(('a' as u32, true))
                } else if c == 'E' {
                    end.x = curr_x;
                    end.y = curr.len();
                    curr.push(('z' as u32, false))
                } else {
                    curr.push((c as u32, false))
                }
            });
        }

        Self {
            data,
            end,
            start,
            x_len,
            y_len,
        }
    }
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
        assert_eq!(part_two(&input), Some(29));
    }
}
