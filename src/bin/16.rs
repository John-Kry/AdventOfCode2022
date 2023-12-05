use petgraph::graph::{DiGraph, Node, NodeIndex};
use petgraph::visit::{Bfs, Dfs, IntoNodeIdentifiers, NodeRef};
use petgraph::{Direction, EdgeDirection, Graph, Outgoing};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Index;

#[derive(Debug)]
struct Valve {
    status: Status,
    flow_rate: u32,
    name: String,
    targets: Vec<String>,
}

#[derive(Debug)]
enum Status {
    Closed,
    Open,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut g = DiGraph::<&Valve, ()>::new();
    let valves: Vec<_> = lines
        .iter()
        .map(|line| {
            let splits: Vec<_> = line.split(&[' ', ',']).collect();

            let valve_name = splits[1];
            let flow_rate = splits[4].split(&['=', ';']).collect::<Vec<&str>>()[1].to_string();
            let targets: Vec<String> = splits[9..]
                .to_vec()
                .iter()
                .filter(|&&c| c.len() > 0)
                .map(|c| c.to_string())
                .collect();

            return Valve {
                status: Status::Closed,
                flow_rate: flow_rate.parse().unwrap(),
                name: valve_name.to_string(),
                targets,
            };
        })
        .collect();
    let mut s_to_idx = HashMap::<String, NodeIndex>::new();
    let mut iter = valves.iter();
    let first = iter.next().unwrap();
    let a = g.add_node(first);
    s_to_idx.insert(first.name.to_string(), a);

    iter.for_each(|v| {
        s_to_idx.insert(v.name.to_string(), g.add_node(v));
    });
    valves.iter().for_each(|v| {
        let idx = s_to_idx.get(&v.name.to_string()).unwrap();
        v.targets.iter().for_each(|t| {
            g.add_edge(idx.to_owned(), s_to_idx.get(t).unwrap().to_owned(), ());
        })
    });


    let mut stack = VecDeque::<(NodeIndex,u32)>::new();
    stack.push_front((a,0));
    let mut set = HashSet::<(u32,NodeIndex)>::new();
    let mut set2 = HashSet::<NodeIndex>::new();
    while let Some((node,depth)) = stack.pop_front(){
        if set2.contains(&node){
            continue;
        }
        let value = (30 -1- depth) * g.node_weight(node).unwrap().flow_rate;
        set.insert((value,node));
        set2.insert(node);
        g.neighbors_directed(node, Outgoing).into_iter().for_each(|n|{
            stack.push_front((n,depth+1));
        });
        dbg!(depth);
        dbg!(node);
    }
    dbg!(set);

    Some(23)
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
