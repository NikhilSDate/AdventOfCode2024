use std::{
    collections::{HashMap, HashSet},
    fs,
};
use std::iter;

use petgraph::{
    graph::NodeIndex,
    graph::{Node, UnGraph},
    Graph,
};

pub fn read_input(path: &str) -> (UnGraph<String, ()>) {
    let content = fs::read_to_string(path).unwrap();
    let mut g = UnGraph::new_undirected();
    let mut nodes = HashMap::new();
    for line in content.lines() {
        let parts: Vec<_> = line.split("-").collect();
        let v1 = parts[0];
        let v2 = parts[1];
        if !nodes.contains_key(v1) {
            nodes.insert(v1, g.add_node(v1.into()));
        }
        if !nodes.contains_key(v2) {
            nodes.insert(v2, g.add_node(v2.into()));
        }
        let v1 = *nodes.get(v1).unwrap();
        let v2 = *nodes.get(v2).unwrap();
        if !g.contains_edge(v1, v2) {
            g.add_edge(v1, v2, ());
        }
    }
    g
}

type Cycle = (NodeIndex, NodeIndex, NodeIndex);

fn sort(cycle: Cycle) -> Cycle {
    let a = cycle.0;
    let b = cycle.1;
    let c = cycle.2;
    let mut v = vec![a, b, c];
    v.sort();
    (v[0], v[1], v[2])
}

fn sort_kn(kn: &HashSet<NodeIndex>) -> Vec<NodeIndex> {
    let mut v = Vec::from_iter(kn.iter().map(|v| *v));
    v.sort();
    v
}

pub fn get_three_cycles(g: &UnGraph<String, ()>) -> HashSet<Cycle> {
    let mut cycles = HashSet::new();
    for node in g.node_indices() {
        let neighbors: Vec<_> = g.neighbors(node).collect();
        for n1 in &neighbors {
            for n2 in &neighbors {
                if g.contains_edge(*n1, *n2) {
                    cycles.insert(sort((node, *n1, *n2)));
                }
            }
        }
    }
    cycles
}

pub fn get_next_Kn(g: &UnGraph<String, ()>, kns: &HashSet<Vec<NodeIndex>>) -> HashSet<Vec<NodeIndex>> {
    let mut subgraphs = HashSet::new();
    for kn in kns {
        // we will try to extend kn by a single node
        let first_node = *kn.iter().next().unwrap();
        let mut extensions: HashSet<NodeIndex> = HashSet::from_iter(g.neighbors(first_node));
        for node in kn {
            let neighbors: HashSet<NodeIndex> = HashSet::from_iter(g.neighbors(*node));
            let intersection = extensions.intersection(&neighbors);
            extensions = HashSet::from_iter(intersection.map(|v| *v));
        }
        for extension in extensions {
            let mut subgraph: Vec<_> = kn.iter().map(|v| *v).collect();
            subgraph.push(extension);
            subgraph.sort();
            subgraphs.insert(subgraph);
        }
    }
    subgraphs
}

pub fn has_historian(g: &UnGraph<String, ()>, node: NodeIndex) -> bool{
    g.node_weight(node).unwrap().starts_with("t")
}

pub fn count_cycles_with_historian(g: &UnGraph<String, ()>, cycles: &HashSet<Cycle>) -> usize {
    cycles.into_iter().filter(|cycle| {has_historian(g, cycle.0) || has_historian(g, cycle.1) || has_historian(g, cycle.2)}).count()
}
