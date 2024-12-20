
use petgraph::graph::NodeIndex;
use petgraph::algo::*;
use graph::UnGraph;
use petgraph::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;

fn main() {
    let g = build_graph();
    let mut total_cost = 0;
    for component in kosaraju_scc(&g) {
        total_cost += calculate_cost(component, &g);
    }
    dbg!(total_cost);
}

fn read_input() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("data/map.txt").unwrap();
    let split = contents.split('\n');
    let mut map: Vec<Vec<char>>= Vec::new();
    let mut lidx = 0;
    for line in split {
        map.push(vec![]);
        for character in line.chars() {
           map.get_mut(lidx).unwrap().push(character);
        }
        lidx = lidx + 1;
    }
    map
}

fn build_graph() -> Graph<(usize, usize), (), Undirected>{
    let map = read_input();
    let mut g = UnGraph::new_undirected();
    let mut indices: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            indices.insert((i, j), g.add_node((i, j)));
        }
    }
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if i > 0 && map[i][j] == map[i - 1][j] {
                g.add_edge(indices[&(i, j)], indices[&(i - 1, j)], ());
            }
            if j > 0 && map[i][j] == map[i][j - 1] {
                g.add_edge(indices[&(i, j)], indices[&(i, j - 1)], ());
            }
            if i < map.len() - 1 && map[i][j] == map[i + 1][j] {
                g.add_edge(indices[&(i, j)], indices[&(i + 1, j)], ());
            }
            if j < map[0].len() - 1 && map[i][j] == map[i][j + 1] {
                g.add_edge(indices[&(i, j)], indices[&(i, j + 1)], ());
            }
        }
    }
    g
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    H, 
    V
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::H => {
                write!(f, "H")
            },
            Direction::V => {
                write!(f, "V")
            }
        }
    }
}

struct FenceNode(Direction, i32, i32);

impl Display for FenceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl From<(Direction, i32, i32)> for FenceNode {
    fn from(value: (Direction, i32, i32)) -> Self {
        FenceNode(value.0, value.1, value.2)
    }
}

use crate::day12::Direction::*;

fn calculate_cost(component: Vec<NodeIndex>, g: &Graph<(usize, usize), (), Undirected> ) -> usize {
    let mut nodes = HashSet::new();
    for index in &component {
        let (ri, rj) = g.node_weight(*index).unwrap();
        let i = *ri as i32;
        let j = *rj as i32;
        nodes.insert((i, j));
    }
    let area = component.len();
    let mut perimeter = 0;
    let mut edge_graph: Graph<(Direction, i32, i32), (), Undirected> = UnGraph::new_undirected();

    let mut edge_nodes: HashMap<(Direction, i32, i32), NodeIndex> = HashMap::new();

    for index in &component {
        let (ri, rj) = g.node_weight(*index).unwrap();
        let i = *ri as i32;
        let j = *rj as i32;
        let mut sides = 0;
        if !nodes.contains(&(i - 1, j)) {
            let neighbor: (Direction, i32, i32) = (H, 2*i, j);
            if !edge_nodes.contains_key(&neighbor) {
                let idx = edge_graph.add_node(neighbor);
                edge_nodes.insert(neighbor, idx); 
            }
            sides += 1;
        }
        if !nodes.contains(&(i + 1, j)) {
            let neighbor = (H, 2*i + 1, j);
            if !edge_nodes.contains_key(&neighbor) {
                let idx = edge_graph.add_node(neighbor);
                edge_nodes.insert(neighbor, idx);
            }
            sides += 1;
        }
        if !nodes.contains(&(i, j - 1)) {
            let neighbor = (V, i, 2*j);
            if !edge_nodes.contains_key(&neighbor) {
                let idx = edge_graph.add_node(neighbor);
                edge_nodes.insert(neighbor, idx);
            }
            sides += 1;
        }
        if !nodes.contains(&(i, j + 1)) {
            let neighbor = (V, i, 2*j + 1);
            if !edge_nodes.contains_key(&neighbor) {
                let idx = edge_graph.add_node(neighbor);
                edge_nodes.insert(neighbor, idx);
            }
            sides += 1;
        }
        perimeter += sides;

        // complete the edge graph
    }
    for (node, idx) in &edge_nodes {
        match node {
            (Direction::H, i, j) => {
                if edge_nodes.contains_key(&(H, *i, *j - 1)) {
                    edge_graph.add_edge(*idx, edge_nodes[&(H, *i, *j - 1)], ());
                }   
                if edge_nodes.contains_key(&(H, *i, *j + 1)) {
                    edge_graph.add_edge(*idx, edge_nodes[&(H, *i, *j + 1)], ());
                }
            },
            (Direction::V, i, j) => {
                if edge_nodes.contains_key(&(V, *i - 1, *j)) {
                    edge_graph.add_edge(*idx, edge_nodes[&(V, *i - 1, *j)], ());
                }   
                if edge_nodes.contains_key(&(V, *i + 1, *j)) {
                    edge_graph.add_edge(*idx, edge_nodes[&(V, *i + 1, *j)], ());
                }
            }
        }
    }
    let components = kosaraju_scc(&edge_graph);
    area * components.len()
}
