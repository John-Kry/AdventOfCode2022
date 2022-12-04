use std::cmp;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let pair = Pair::try_from(line).expect("string should be in parseable format");
                if pair.items_contain_each_other() {
                    return Some(pair);
                }
                return None;
            })
            .count()
            .try_into()
            .expect("usize shouldn't be that large"),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let pair = Pair::try_from(line).expect("string should be in parseable format");
                if pair.does_overlap() {
                    return Some(pair);
                }
                return None;
            })
            .count()
            .try_into()
            .expect("usize shouldn't be that large"),
    )
}

#[derive(Debug)]
struct Pair {
    one: Interval,
    two: Interval,
}

#[derive(Debug)]
struct Interval {
    left: u32,
    right: u32,
}

impl Pair {
    fn items_contain_each_other(self: &Self) -> bool {
        if self.one.left <= self.two.left && self.one.right >= self.two.right {
            return true;
        }
        if self.two.left <= self.one.left && self.two.right >= self.one.right {
            return true;
        }
        false
    }

    fn does_overlap(self: &Self) -> bool {
        if self.one.left <= self.two.left
            && self.one.right >= cmp::min(self.two.left, self.two.right)
        {
            return true;
        }

        if self.two.left <= self.one.left
            && self.two.right >= cmp::min(self.one.left, self.one.right)
        {
            return true;
        }

        return false;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

impl TryFrom<&str> for Pair {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((interval_one_str, interval_two_str)) = value.split_once(',') {
            return Ok(Self {
                one: interval_one_str.parse()?,
                two: interval_two_str.parse()?,
            });
        } else {
            Err(())
        }
    }
}

impl FromStr for Interval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').expect("Aoc input should be valid");
        Ok(Self {
            left: left.parse().unwrap(),
            right: right.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }
    #[test]
    fn test_one() {
        assert!(Pair {
            one: Interval { left: 3, right: 7 },
            two: Interval { left: 2, right: 8 }
        }
        .items_contain_each_other());
        assert!(Pair {
            one: Interval { left: 3, right: 7 },
            two: Interval { left: 2, right: 8 }
        }
        .items_contain_each_other());
        assert!(Pair::try_from("3-3,3-3")
            .unwrap()
            .items_contain_each_other());
        assert!(Pair::try_from("12-75,32-74").unwrap().does_overlap())
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
