use graph::UnGraph;
use petgraph::algo::*;
use petgraph::graph::NodeIndex;
use petgraph::*;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
pub struct Map {
    g: Graph<(usize, usize), (), Undirected>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,

}

pub fn read_input() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("data/day20/example.txt").unwrap();
    let split = contents.split('\n');
    let mut map: Vec<Vec<char>> = Vec::new();
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

pub fn build_graph(map: Vec<Vec<char>>) -> Map {
    let mut g = UnGraph::new_undirected();
    let mut indices: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' || map[i][j] == 'S' || map[i][j] == 'E' {
                indices.insert((i, j), g.add_node((i, j)));
            }
            if map[i][j] == 'S' {
                start = (i, j);
            }
            if map[i][j] == 'E' {
                end = (i, j);                
            }
        }
    }
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if i > 0 && map[i][j] == '.' && map[i - 1][j] == '.' {
                g.add_edge(indices[&(i, j)], indices[&(i - 1, j)], ());
            }
            if j > 0 && map[i][j] == '.' && map[i][j - 1]  == '.' {
                g.add_edge(indices[&(i, j)], indices[&(i, j - 1)], ());
            }
            if i < map.len() - 1 && map[i][j] == '.' && map[i + 1][j] == '.' {
                g.add_edge(indices[&(i, j)], indices[&(i + 1, j)], ());
            }
            if j < map[0].len() - 1 && map[i][j] == '.' && map[i][j + 1] == '.' {
                g.add_edge(indices[&(i, j)], indices[&(i, j + 1)], ());
            }
        }
    }
    Map {
        g,
        start,
        end,
        height: map.len(),
        width: map[0].len()
    }
}

pub fn run_search(map: &Map, start: NodeIndex) -> HashMap<NodeIndex, usize>  {
    let mut distances = HashMap::new();
    let g = &map.g;
    let mut q = VecDeque::new();
    q.push_back(start);
    distances.insert(start, 0);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for n in g.neighbors(v) {
            if !distances.contains_key(&n) {
                distances.insert(n, distances.get(&v).unwrap()+ 1);
            }
        }
    }
    distances
}

pub fn calculate_cheats(map: &Map) {
    let mut indices = HashMap::new();
    let g = &map.g;
    for node in g.node_indices() {
        indices.insert(g.node_weight(node).unwrap(), node);
    }
    let forward_distances = run_search(map, *indices.get(&map.start).unwrap());
    let reverse_distances = run_search(map, *indices.get(&map.end).unwrap());

    let baseline = forward_distances.get(indices.get(&map.end).unwrap()).unwrap();

    for i in 0..map.height {
        for j in 0..map.width {
            if !indices.contains_key(&(i, j)) {
                // we can try to cheat by skipping this wall
            }
        }
    }

}