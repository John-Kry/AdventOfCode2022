use std::cmp;
use std::fmt::Debug;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let x = lines.clone().next().unwrap().len();
    let y = lines.count();
    let mut grid = Grid {
        data: vec![],
        x_len: x,
        y_len: y,
    };

    create_grid(&mut grid, x, input);

    let mut count = 0;
    for curr_x in 0..x {
        for curr_y in 0..y {
            if grid.is_visible(curr_x as i32, curr_y as i32) {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let x = lines.clone().next().unwrap().len();
    let y = lines.count();
    let mut grid = Grid {
        data: vec![],
        x_len: x,
        y_len: y,
    };

    create_grid(&mut grid, x, input);

    let mut max = 0;
    for curr_x in 0..x {
        for curr_y in 0..y {
            max = cmp::max(grid.get_scenic_score(curr_x as i32, curr_y as i32), max);
        }
    }
    Some(max as u32)
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<(u8, bool)>>,
    x_len: usize,
    y_len: usize,
}

impl Grid {
    // This one is even worse
    fn is_visible(&self, x: i32, y: i32) -> bool {
        let mut curr_x: i32 = x as i32;
        let mut curr_y: i32 = y as i32;
        let size = self.get_value(x, y).0;
        // left
        curr_x -= 1;
        while curr_x >= 0 {
            if self.get_value(curr_x, curr_y).0 >= size {
                break;
            }
            curr_x -= 1;
        }
        if curr_x == -1 {
            return true;
        }
        //right
        curr_x = x;
        curr_x += 1;
        while curr_x < self.x_len as i32 {
            if self.get_value(curr_x, curr_y).0 >= size {
                break;
            }
            curr_x += 1;
        }
        if curr_x == self.x_len as i32 {
            return true;
        }
        curr_x = x;
        //down
        curr_y += 1;
        while curr_y < self.y_len as i32 {
            if self.get_value(curr_x, curr_y).0 >= size {
                break;
            }
            curr_y += 1;
        }
        if curr_y == self.y_len as i32 {
            return true;
        }
        // up
        curr_y = y;
        curr_y -= 1;
        while curr_y >= 0 {
            if self.get_value(curr_x, curr_y).0 >= size {
                break;
            }
            curr_y -= 1;
        }
        if curr_y == -1 {
            return true;
        }
        return false;
    }
    fn get_value(&self, x: i32, y: i32) -> &(u8, bool) {
        self.data.get(x as usize).unwrap().get(y as usize).unwrap()
    }

    // This fn is terrible, don't look
    fn get_scenic_score(&mut self, x: i32, y: i32) -> i32 {
        let mut curr_x: i32 = x as i32;
        let mut curr_y: i32 = y as i32;
        let size = self.get_value(x, y).0;
        // left
        curr_x -= 1;
        while curr_x >= 0 {
            if self.get_value(curr_x, curr_y).0 >= size {
                curr_x -= 1;
                break;
            }
            curr_x -= 1;
        }
        let score1 = x - (curr_x + 1);

        //right
        curr_x = x;
        curr_x += 1;
        while curr_x < self.x_len as i32 {
            if self.get_value(curr_x, curr_y).0 >= size {
                curr_x += 1;
                break;
            }
            curr_x += 1;
        }

        // curr_x 5
        // x 3
        let score2 = (curr_x - 1) - x;
        curr_x = x;
        //down
        curr_y += 1;
        while curr_y < self.y_len as i32 {
            if self.get_value(curr_x, curr_y).0 >= size {
                curr_y += 1;
                break;
            }
            curr_y += 1;
        }
        let score4 = (curr_y - 1) - y;
        // up
        curr_y = y;
        curr_y -= 1;
        while curr_y >= 0 {
            if self.get_value(curr_x, curr_y).0 >= size {
                curr_y -= 1;
                break;
            }
            curr_y -= 1;
        }
        let score3 = y - (curr_y + 1);

        score1 * score2 * score3 * score4
    }
}

fn create_grid(grid: &mut Grid, x: usize, input: &str) {
    let lines = input.lines();
    for curr_x in 0..x {
        grid.data.push(vec![]);
        let curr = grid.data.get_mut(curr_x).unwrap();
        lines.clone().for_each(|line| {
            curr.push((
                (line.as_bytes().get(curr_x).unwrap().clone() as char)
                    .to_digit(10)
                    .unwrap() as u8,
                false,
            ))
        })
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
