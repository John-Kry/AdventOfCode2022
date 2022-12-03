use std::collections::HashSet;
use std::str::Chars;

pub fn part_one(input: &str) -> Option<u32> {
    let mut seen: HashSet<char> = HashSet::new();
    Some(input.lines().fold(0u32, |prev, line| {
        let half = line.len() / 2;
        let (backpack_1, backpack_2) = line.split_at(half);

        backpack_1.chars().for_each(|c| {
            seen.insert(c);
        });

        let dupe = backpack_2.chars().find(|c| seen.contains(c)).unwrap();

        seen.clear();

        prev + priority(dupe)
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut sum = 0;
    while let Some(x) = lines.next() {
        let val = priority(
            Backpacks {
                one: x.chars(),
                two: lines.next().unwrap().chars(),
                three: lines.next().unwrap().chars(),
            }
            .common_char(),
        );
        sum += val;
    }

    Some(sum)
}

struct Backpacks<'a> {
    one: Chars<'a>,
    two: Chars<'a>,
    three: Chars<'a>,
}
impl<'a> Backpacks<'a> {
    fn common_char(self) -> char {
        let mut seen: HashSet<char> = HashSet::new();
        self.one.for_each(|c| {
            seen.insert(c);
        });

        let mut seen_2: HashSet<char> = HashSet::new();
        self.two.for_each(|c| {
            seen_2.insert(c);
        });

        seen.clone().iter().for_each(|c| {
            if !seen_2.contains(c) {
                seen.remove(c);
            }
        });

        let dupe = self.three.clone().find(|c| seen.contains(c));
        dupe.unwrap()
    }
}

fn priority(c: char) -> u32 {
    let mut val = c as u32;
    if c.is_ascii_lowercase() {
        val -= 96;
    } else {
        val -= 38;
    }
    val
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
