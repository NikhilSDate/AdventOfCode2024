use petgraph::graph::NodeIndex;
use petgraph::algo::*;
use graph::UnGraph;
use petgraph::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
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
    for index in &component {
        let (ri, rj) = g.node_weight(*index).unwrap();
        let i = *ri as i32;
        let j = *rj as i32;
        let mut sides = 0;
        if !nodes.contains(&(i - 1, j)) {
            sides += 1;
        }
        if !nodes.contains(&(i + 1, j)) {
            sides += 1;
        }
        if !nodes.contains(&(i, j - 1)) {
            sides += 1;
        }
        if !nodes.contains(&(i, j + 1)) {
            sides += 1;
        }
        perimeter += sides;
    }
    area * perimeter
}
