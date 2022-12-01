use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let elves: Vec<&str>= input.split("\n\n").collect();

    return Some(elves.iter().fold(0, |acc, elf_lines| {
        let calories = elf_lines.lines().map(|lin| {
            lin.parse::<u32>().unwrap()
        }).sum();
        return cmp::max(acc, calories);
    }));
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: Vec<&str>= input.split("\n\n").collect();

    let mut sums: Vec<u32> = elves.iter().map(|elf_lines|{
        return elf_lines.lines().map(|lin|{
            lin.parse::<u32>().unwrap()
        }).sum();
    }).collect();

    sums.sort();
    sums.reverse();
    Option::from(sums.into_iter().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
