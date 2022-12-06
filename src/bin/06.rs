use std::collections::{HashMap};

pub fn part_one(input: &str) -> Option<usize> {
    get_first_unique(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    get_first_unique(input, 14)
}

fn get_first_unique(input: &str, size:usize)->Option<usize>{
    let mut seen = Seen::new();
    let mut left =0;
    let mut right = 0;
    let chars = input.as_bytes();
    while right < chars.len(){
        while seen.get(chars[right] as char) > &0_usize {
            seen.remove(chars[left] as char);
            left+=1;
        }
        if (right - left) == size {
            return Some(right);
        }
        seen.add(chars[right] as char);
        right+=1;
    }
    None
}

struct Seen{
    map: HashMap<char, usize>
}


impl Seen {
    fn new()->Self{
       Self{
           map: HashMap::new()
       }
    }

    fn get(&self, c:char) -> &usize {
        self.map.get(&c).unwrap_or(&0_usize)
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
