pub fn part_one(input: &str) -> Option<u32> {
    let mut calories = parse_input(input);
    calories.sort_unstable();
    calories.into_iter().rev().next()
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .into_iter()
        .map(|elf_lines| {
            return elf_lines
                .lines()
                .map(|lin| lin.parse::<u32>().unwrap())
                .sum();
        })
        .collect::<Vec<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = parse_input(input);

    calories.sort_unstable();
    Some(calories.into_iter().rev().take(3).sum())
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
