use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .filter_map(|line| {
            if let Some((interval_one_str, interval_two_str)) = line.split_once(',') {
                let pog = interval_one_str.split_once('-');
                let interval_one = Interval {
                    left: pog.unwrap().0.parse::<u32>().unwrap(),
                    right: pog.unwrap().1.parse::<u32>().unwrap(),
                };
                let champ = interval_two_str.split_once('-');
                let interval_two = Interval {
                    left: champ.unwrap().0.parse::<u32>().unwrap(),
                    right: champ.unwrap().1.parse::<u32>().unwrap(),
                };
                let pair = Pair {
                    one: interval_one,
                    two: interval_two,
                };
                if pair.has_fully_contain() {
                    return Some(pair);
                }
            }
            None
        })
        .collect::<Vec<Pair>>();
    Some(pairs.len() as u32)
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
    fn has_fully_contain(self: &Self) -> bool {
        if self.one.left <= self.two.left && self.one.right >= self.two.right {
            return true;
        }
        if self.two.left <= self.one.left && self.two.right >= self.one.right {
            return true;
        }
        false
    }

    // &self readonly ref
    fn overlaps_at_all(self: &Self) -> bool{
        if self.one.left <= self.two.left && self.one.right >= cmp::min(self.two.left, self.two.right){
            return true;
        }

        if self.two.left <= self.one.left && self.two.right >= cmp::min(self.one.left, self.one.right){
            return true;
        }

        return false;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .filter_map(|line| {
                let pair = Pair::try_from(line).unwrap();
                if pair.overlaps_at_all() {
                    return Some(pair);
                }
                return None
        })
        .collect::<Vec<Pair>>();
    Some(pairs.len() as u32)
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
                let pog = interval_one_str.split_once('-');
                let interval_one = Interval {
                    left: pog.unwrap().0.parse::<u32>().unwrap(),
                    right: pog.unwrap().1.parse::<u32>().unwrap(),
                };
                let champ = interval_two_str.split_once('-');
                let interval_two = Interval {
                    left: champ.unwrap().0.parse::<u32>().unwrap(),
                    right: champ.unwrap().1.parse::<u32>().unwrap(),
                };
                return Ok(Pair {
                    one: interval_one,
                    two: interval_two,
                });
            }else {
             Err(())
         }
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

        // [3,7] [2,8]
        //[2,8] [3,7]
        // assert!(Pair{ one: Interval{ left:3 , right: 7 }, two: Interval { left: 2, right: 8 } }.overlaps_at_all());
        // assert!(Pair{ one: Interval{ left:3 , right: 7 }, two: Interval { left: 2, right: 8 } }.overlaps_at_all());
        // assert!(Pair::try_from("3-3,3-3").unwrap().overlaps_at_all());
        assert!(Pair::try_from("12-75,32-74").unwrap().overlaps_at_all())
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
