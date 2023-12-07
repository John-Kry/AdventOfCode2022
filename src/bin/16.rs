use itertools::Itertools;
use petgraph::algo::floyd_warshall;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::NodeRef;
use petgraph::{Graph, Outgoing};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::ops::Index;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: i32,
    name: String,
    targets: Vec<String>,
}

#[derive(Debug, Clone)]
enum Status {
    Closed,
    Open,
}

pub fn part_one(input: &str) -> Option<i32> {
    let valves = generate_valves(input);
    let g = ValveGraph::new(valves);

    let mut q: VecDeque<(String, i32, i32, HashSet<String>)> = VecDeque::new();
    q.push_front(("AA".to_string(), 30, 0, HashSet::new()));

    let mut max_set: HashMap<String, i32> = HashMap::new();
    while let Some((current_valve, time_remaining, total_relief, opened)) = q.pop_front() {
        for (node, steps) in g.get_others(current_valve.as_str()).iter() {
            let v = g.weight(node.clone());
            let t_remain = time_remaining - 1 - steps.clone();
            if t_remain.is_negative() || v.flow_rate == 0 {
                continue;
            }
            let mut o = opened.clone();
            if o.contains(v.name.as_str()) {
                continue;
            }
            let t_relief = total_relief + (t_remain * v.flow_rate);
            let hash = o.iter().join("");
            match max_set.get_mut(&hash) {
                None => {
                    max_set.insert(hash, t_relief);
                }
                Some(x) => {
                    if t_relief > x.clone() {
                        *x = t_relief;
                    }
                }
            }
            o.insert(v.name.clone());
            q.push_front((v.name.clone(), t_remain, t_relief, o));
        }
    }
    Some(max_set.values().max().unwrap().clone())
}

struct ValveGraph<T> {
    g: Graph<T, ()>,
    map: HashMap<String, NodeIndex>,
    pub node_to_node: HashMap<(NodeIndex, NodeIndex), i32>,
}

impl ValveGraph<Valve> {
    fn new(valves: Vec<Valve>) -> Self {
        let mut g = DiGraph::<Valve, ()>::new();
        let mut s_to_idx = HashMap::<String, NodeIndex>::new();

        valves.iter().for_each(|v| {
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
            .get(&(self.get_index(s1.as_str()), self.get_index(s2.as_str())))
            .unwrap();
    }
    fn weight(&self, i: NodeIndex) -> &Valve {
        return self.g.node_weight(i).unwrap();
    }

    fn get_neighbors(&self, s: &str) -> Vec<(NodeIndex, &Valve)> {
        let i = self.get_index(s);
        return self
            .g
            .neighbors_directed(i, Outgoing)
            .map(|n| (n, self.g.index(n)))
            .collect::<Vec<_>>();
    }

    fn get_others(&self, s: &str) -> Vec<(NodeIndex, &i32)> {
        let i = self.get_index(s);
        return self
            .g
            .node_indices()
            .map(|x| (x, self.node_to_node.get(&(i, x)).unwrap()))
            .collect::<Vec<_>>();
    }

    fn get_index(&self, s: &str) -> NodeIndex {
        return self.map.get(s).unwrap().clone();
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let valves = generate_valves(input);

    let g = ValveGraph::new(valves);

    let mut q: VecDeque<(String, i32, i32, MyHash)> = VecDeque::new();
    q.push_front(("AA".to_string(), 26, 0, MyHash::new()));

    let mut max_set: HashMap<MyHash, (i32, MyHash)> = HashMap::new();
    while let Some((current_valve, time_remaining, total_relief, opened)) = q.pop_front() {
        for (node, steps) in g.get_others(current_valve.as_str()).iter() {
            let v = g.weight(node.clone());
            let t_remain = time_remaining - 1 - steps.clone();
            if t_remain.is_negative() || v.flow_rate == 0 {
                continue;
            }
            if opened.h.contains(v.name.as_str()) {
                continue;
            }
            let t_relief = total_relief + (t_remain * v.flow_rate);
            let mut o = opened.clone();
            o.h.insert(v.name.clone());
            // IT DOES NEED TO BE ORDERED WTF
            match max_set.get_mut(&o.clone()) {
                None => {
                    max_set.insert(o.clone(), (t_relief, o.clone()));
                }
                Some(x) => {
                    if t_relief > x.0 {
                        *x = (t_relief, o.clone());
                    }
                }
            }
            q.push_front((v.name.clone(), t_remain, t_relief, o.clone()));
        }
    }
    let mut max = 0;

    max_set
        .values()
        .tuple_combinations::<(_, _)>()
        .filter(|(a, b)| {
            return a.1.h.is_disjoint(&b.1.h);
        })
        .for_each(|(a, b)| {
            max = max.max(a.0 + &b.0);
        });

    Some(max)
}

#[derive(Eq, PartialEq, Clone)]
struct MyHash {
    pub h: HashSet<String>,
}

impl MyHash {
    fn new() -> Self {
        Self { h: HashSet::new() }
    }
}

impl Hash for MyHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = self.h.iter().sorted().join("");
        s.hash(state);
    }
}
fn generate_valves(input: &str) -> Vec<Valve> {
    let lines: Vec<_> = input.lines().collect();
    return lines
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
                flow_rate: flow_rate.parse().unwrap(),
                name: valve_name.to_string(),
                targets,
            };
        })
        .collect();
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
        assert_eq!(part_two(&input), Some(1707));
    }
}
