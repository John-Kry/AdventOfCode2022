use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::Command::{CD, LS};
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let mut root = Rc::new(RefCell::new(Node::new()));
    let mut curr_node =  Rc::clone(&root);
    let mut i = 0;
    while i < input.lines().count() {
        if i ==7{
            break;
        }
        let mut line = input.lines().nth(i).unwrap();
        let command: Command = line.parse().unwrap();
        dbg!(&command);
        match command {
            CD => {
                let location = line.split(' ') .nth(2).unwrap().chars().nth(0).unwrap();
                dbg!(location);
                if location == '/' {
                    curr_node = curr_node.borrow_mut().get_mut().root();
                } else if location  == '.' {
                   curr_node = curr_node.into_inner().prev.unwrap()
                }
                else {
                    curr_node = curr_node.into_inner().cd(location);
                }
            }
            LS => {
                i+=1;
                line = input.lines().nth(i).unwrap();

                while !line.contains('$') {
                    let (size_or_dir, name) = line.split_once(' ').unwrap();
                    dbg!(size_or_dir);
                    dbg!(name);
                    match size_or_dir {
                        "dir" => curr_node.into_inner().children.push(Rc::new(RefCell::new(Node {
                            label: name.parse().unwrap(),
                            children: vec![],
                            files: vec![],
                            prev: curr_node.into_inner().prev
                        }))),
                        other => curr_node.borrow_mut().into_inner().files.push(File {
                            size: other.parse().unwrap(),
                            name: name.to_string(),
                        }),
                    }
                    i += 1;
                    line = input.lines().nth(i).unwrap();
                }
            }
        }
        i+=1;
    }
    dbg!(root);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
enum Command {
    CD,
    LS,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("cd") {
            return Ok(CD);
        }
        if s.contains("ls") {
            return Ok(LS);
        }
         dbg!(s);
        Err(())
    }
}

#[derive(Clone, Debug)]
struct Node {
    label: char,
    children: Vec<Rc<RefCell<Node>>>,
    files: Vec<File>,
    prev: Option<Rc<RefCell<Node>>>
}
#[derive(Debug, Clone)]
struct File {
    size: u32,
    name: String,
}
impl Node {
    pub fn new() -> Self {
        Node {
            label: '/',
            children: Vec::new(),
            files: Vec::new(),
            prev: None
        }
    }

    pub fn root(&self) -> Rc<RefCell<Node>>{
        let mut curr_node = self.clone();
        while curr_node.prev.is_some(){
            curr_node = curr_node.prev.unwrap().into_inner()
        }
        Rc::new(RefCell::new(curr_node))
    }

    pub fn cd(&self, c: char) -> Rc<RefCell<Node>> {
        self
            .children.clone()
            .iter()
            .find(|node| node.into_inner().label == c)
            .expect("Directory exists").clone()
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
