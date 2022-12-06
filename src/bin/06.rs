use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<usize> {
    get_first_unique(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    get_first_unique(input, 14)
}

fn get_first_unique(input: &str, size:usize)->Option<usize>{
    let mut seen = Seen::new();
    for (i, c) in input.chars().enumerate() {
        if i < size {
            seen.add(c);
        } else {
            seen.add(c);
            seen.remove(input.chars().nth(i - size).unwrap());
            if seen.is_unique(c) {
                return Some(i + 1_usize);
            }
        }
    }
    None
}

struct Seen{
    pub map: HashMap<char, usize>
}


impl Seen {
    fn new()->Self{
       Self{
           map: HashMap::new()
       }
    }
    fn is_unique(&self, c:char) ->bool{
       return self.map.iter().all(|(c,i)|{
           i <= &1_usize
       });
    }
   fn add(&mut self, c:char){
       if self.map.contains_key(&c) {
           self.map.insert(c,self.map.get(&c).expect("Should exist!") +1);
       }else{
           self.map.insert(c, 1);
       }
   }
    fn remove(&mut self, c:char){
        self.map.insert(c,self.map.get(&c).expect("Should exist") -1);
    }
}



fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_one(&input), Some(7));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(&input), Some(5));

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(&input), Some(6));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(&input), Some(10));

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_two(&input), Some(29));
    }
}
