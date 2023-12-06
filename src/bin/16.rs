use petgraph::algo::floyd_warshall;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{IntoNodeIdentifiers, NodeRef};
use petgraph::{Graph, Outgoing};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Valve {
    status: Status,
    flow_rate: u32,
    name: String,
    targets: Vec<String>,
}

#[derive(Debug, Clone)]
enum Status {
    Closed,
    Open,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
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

    let g = ValveGraph::new(valves);

    let mut q: VecDeque<(String, u32, u32, HashSet<String>)> = VecDeque::new();
    q.push_front(("AA".to_string(), 30, 0, HashSet::new()));

    let mut max_set: HashMap<String, u32> = HashMap::new();
    while let Some((current_valve, time_remaining, total_relief, opened)) = q.pop_front() {
        let neighbors = g.get_neighbors(current_valve);
        for (n, v) in neighbors.iter() {
            let t_remain = time_remaining - 1 - steps;
            let mut o = opened.clone();
            if { o.contains(v.name.as_str()) } {
                continue;
            }
            let t_relief = total_relief + (t_remain * v.flow_rate);
            let hash = o.iter().join("");
            match max_set.get(&hash) {
                None => {
                    max_set.insert(hash, t_relief);
                }
                Some(&x) => {
                    if t_relief > x {
                        max_set.insert(hash, t_relief);
                    }
                }
            }
            o.insert(v.name.clone());
            q.push_front((v.name.clone(), t_remain, t_relief, o));
        }
    }
    dbg!(&max_set);

    Some(max_set.values().max().unwrap().clone())
}

struct ValveGraph<T> {
    g: Graph<T, ()>,
    map: HashMap<String, NodeIndex>,
    node_to_node: HashMap<(NodeIndex, NodeIndex), i32>,
}

impl ValveGraph<Valve> {
    fn new(valves: Vec<Valve>) -> Self {
        let mut g = DiGraph::<Valve, ()>::new();
        let mut s_to_idx = HashMap::<String, NodeIndex>::new();
        let mut iter = valves.iter();
        let first = iter.next().unwrap();
        let a = g.add_node(first.clone());
        s_to_idx.insert(first.name.to_string(), a);

        iter.for_each(|v| {
            s_to_idx.insert(v.name.to_string(), g.add_node(v.clone()));
        });
        valves.iter().for_each(|v| {
            let idx = s_to_idx.get(&v.name.to_string()).unwrap();
            v.targets.iter().for_each(|t| {
                g.add_edge(idx.to_owned(), s_to_idx.get(t).unwrap().to_owned(), ());
            })
        });
        let fw = floyd_warshall(&g, |_| 1).unwrap();
        ValveGraph {
            g,
            map: s_to_idx,
            node_to_node: fw,
        }
    }
    fn dist(&self, s1: String, s2: String) -> &i32 {
        return self
            .node_to_node
            .get(&(self.get_index(s1), self.get_index(s2)))
            .unwrap();
    }

    fn get_neighbors(&self, s: String) -> Vec<(NodeIndex, &Valve)> {
        let i = self.get_index(s);
        return self
            .g
            .neighbors_directed(i, Outgoing)
            .map(|n| (n, self.g.index(n)))
            .collect::<Vec<_>>();
    }

    fn get_index(&self, s: String) -> NodeIndex {
        return self.map.get(s.as_str()).unwrap().clone();
    }
}

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
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
