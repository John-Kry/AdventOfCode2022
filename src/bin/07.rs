use crate::Command::{CD, LS};
use id_tree::InsertBehavior::AsRoot;
use id_tree::*;
use std::collections::HashSet;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let mut tree: Tree<MyNode> = TreeBuilder::new().build();
    let root_id: NodeId = tree
        .insert(
            Node::new(MyNode {
                label: "/".to_string(),
                files: vec![],
            }),
            AsRoot,
        )
        .unwrap();

    let mut curr_node = root_id.clone();
    let mut i = 0;
    while i < input.lines().count() {
        let mut line = input.lines().nth(i).unwrap();
        let command: Command = line.parse().unwrap();
        match command {
            CD => {
                let location = line.split(' ').nth(2).unwrap();
                dbg!(location);
                dbg!(tree.get(&curr_node).unwrap().data());
                if location == "/" {
                    curr_node = root_id.clone();
                } else if location == ".." {
                    curr_node = tree.get(&curr_node).unwrap().parent().unwrap().clone();
                } else {
                    curr_node = tree
                        .get(&curr_node)
                        .unwrap()
                        .children()
                        .iter()
                        .find(|x| tree.get(x).unwrap().data().label == location)
                        .unwrap()
                        .clone();
                }
            }
            LS => {
                i += 1;
                line = input.lines().nth(i).unwrap();

                loop {
                    let (size_or_dir, name) = line.split_once(' ').unwrap();
                    dbg!(size_or_dir);
                    dbg!(name);
                    match size_or_dir {
                        "dir" => {
                            tree.insert(
                                Node::new(MyNode {
                                    label: name.to_string(),
                                    files: vec![],
                                }),
                                InsertBehavior::UnderNode(&curr_node),
                            )
                            .expect("TODO: panic message");
                        }
                        other => tree
                            .get_mut(&curr_node)
                            .unwrap()
                            .data_mut()
                            .files
                            .push(File {
                                size: other.parse().unwrap(),
                                name: name.to_string(),
                            }),
                    }
                    i += 1;
                    if i == input.lines().count() || input.lines().nth(i).unwrap().contains('$'){
                        i-=1;
                        break;
                    }
                    line = input.lines().nth(i).unwrap();
                }
            }
        }
        i += 1;
    }
    let mut s = String::new();
    &tree.write_formatted(&mut s);
    println!("{}",s);

    Some(
        tree.traverse_pre_order(tree.root_node_id().unwrap())
            .unwrap()
            .inspect(|s|{
                dbg!(s.data());
            })
            .map(|n| total_size(&tree, n))
            .filter(|n|n <=&100_000)
            .inspect(|s| {
                dbg!(s);
            })
            .map(|i|i as u32)
            .sum::<u32>(),
    )
}
fn total_size(tree: &Tree<MyNode>, node: &Node<MyNode>) -> u32{
    let mut total = node.data().files.iter().fold(0, |s, i| s + i.size);
    for child in node.children() {
        total += total_size(tree, tree.get(child).unwrap());
    }
    total
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tree: Tree<MyNode> = TreeBuilder::new().build();
    let root_id: NodeId = tree
        .insert(
            Node::new(MyNode {
                label: "/".to_string(),
                files: vec![],
            }),
            AsRoot,
        )
        .unwrap();

    let mut curr_node = root_id.clone();
    let mut i = 0;
    while i < input.lines().count() {
        let mut line = input.lines().nth(i).unwrap();
        let command: Command = line.parse().unwrap();
        match command {
            CD => {
                let location = line.split(' ').nth(2).unwrap();
                dbg!(location);
                dbg!(tree.get(&curr_node).unwrap().data());
                if location == "/" {
                    curr_node = root_id.clone();
                } else if location == ".." {
                    curr_node = tree.get(&curr_node).unwrap().parent().unwrap().clone();
                } else {
                    curr_node = tree
                        .get(&curr_node)
                        .unwrap()
                        .children()
                        .iter()
                        .find(|x| tree.get(x).unwrap().data().label == location)
                        .unwrap()
                        .clone();
                }
            }
            LS => {
                i += 1;
                line = input.lines().nth(i).unwrap();

                loop {
                    let (size_or_dir, name) = line.split_once(' ').unwrap();
                    dbg!(size_or_dir);
                    dbg!(name);
                    match size_or_dir {
                        "dir" => {
                            tree.insert(
                                Node::new(MyNode {
                                    label: name.to_string(),
                                    files: vec![],
                                }),
                                InsertBehavior::UnderNode(&curr_node),
                            )
                                .expect("TODO: panic message");
                        }
                        other => tree
                            .get_mut(&curr_node)
                            .unwrap()
                            .data_mut()
                            .files
                            .push(File {
                                size: other.parse().unwrap(),
                                name: name.to_string(),
                            }),
                    }
                    i += 1;
                    if i == input.lines().count() || input.lines().nth(i).unwrap().contains('$'){
                        i-=1;
                        break;
                    }
                    line = input.lines().nth(i).unwrap();
                }
            }
        }
        i += 1;
    }
    let mut s = String::new();
    &tree.write_formatted(&mut s);
    println!("{}",s);

    let total_space = 70000000_u32;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap()).unwrap());
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30000000_u32;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    Some(
        tree.traverse_pre_order(tree.root_node_id().unwrap())
            .unwrap()
            .map(|n| total_size(&tree, n))
            .filter(|n|n >=&minimum_space_to_free)
            .min().unwrap()
    )
}

fn generate_tree()

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
struct MyNode {
    label: String,
    files: Vec<File>,
}
#[derive(Debug, Clone)]
struct File {
    size: u32,
    name: String,
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
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
