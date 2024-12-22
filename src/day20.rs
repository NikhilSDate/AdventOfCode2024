use graph::UnGraph;
use petgraph::algo::*;
use petgraph::graph::NodeIndex;
use petgraph::*;
use std::cmp::min;
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
    let contents = fs::read_to_string("data/day20/data.txt").unwrap();
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
    let mut map = map.clone();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                start = (i, j);
            }
            if map[i][j] == 'E' {
                end = (i, j);
            }
            if map[i][j] == '.' || map[i][j] == 'S' || map[i][j] == 'E' {
                indices.insert((i, j), g.add_node((i, j)));
                map[i][j] = '.';
            }
        }
    }
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if i > 0 && map[i][j] == '.' && map[i - 1][j] == '.' {
                g.add_edge(indices[&(i, j)], indices[&(i - 1, j)], ());
            }
            if j > 0 && map[i][j] == '.' && map[i][j - 1] == '.' {
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
        width: map[0].len(),
    }
}

pub fn run_search(map: &Map, start: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let g = &map.g;
    let mut q = VecDeque::new();
    q.push_back(start);
    distances.insert(start, 0);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for n in g.neighbors(v) {
            if !distances.contains_key(&n) {
                distances.insert(n, distances.get(&v).unwrap() + 1);
                q.push_back(n);
            }
        }
    }
    distances
}

pub fn get_neighbors((i, j): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i > 0 {
        result.push((i - 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if i < height - 1 {
        result.push((i + 1, j));
    }
    if j < width - 1 {
        result.push((i, j + 1));
    }
    result
}

pub fn calculate_improvements(map: &Map) -> HashMap<((usize, usize), (usize, usize)), usize> {
    let mut indices = HashMap::new();
    let g = &map.g;
    for node in g.node_indices() {
        indices.insert(g.node_weight(node).unwrap(), node);
    }
    let forward_distances = run_search(map, *indices.get(&map.start).unwrap());
    let reverse_distances = run_search(map, *indices.get(&map.end).unwrap());
    let baseline = *forward_distances
        .get(indices.get(&map.end).unwrap())
        .unwrap();
    let mut improvements = HashMap::new();
    for i in 0..map.height {
        for j in 0..map.width {
            if !indices.contains_key(&(i, j)) {
                // we can try to cheat by skipping this wall
                let neighbors = get_neighbors((i, j), map.height, map.width);
                for m in 0..neighbors.len() {
                    for n in 0..neighbors.len() {
                        let (i1, j1) = neighbors[m];
                        let (i2, j2) = neighbors[n];
                        if (i1, j1) != (i2, j2)
                            && indices.contains_key(&(i1, j1))
                            && indices.contains_key(&(i2, j2))
                        {
                            let mut dist = *forward_distances
                                .get(indices.get(&(i1, j1)).unwrap())
                                .unwrap();
                            dist += 2;
                            dist += reverse_distances
                                .get(indices.get(&(i2, j2)).unwrap())
                                .unwrap();
                            if baseline > dist {
                                improvements.insert(((i1, j1), (i2, j2)), baseline - dist);
                            }
                        }
                    }
                }
            }
        }
    }
    improvements
}

pub fn calculate_long_cheat_improvements(
    map: &Map,
    limit: usize,
) -> HashMap<((usize, usize), (usize, usize)), usize> {
    let mut indices = HashMap::new();
    let g = &map.g;
    for node in g.node_indices() {
        indices.insert(g.node_weight(node).unwrap(), node);
    }
    let forward_distances = run_search(map, *indices.get(&map.start).unwrap());
    let reverse_distances = run_search(map, *indices.get(&map.end).unwrap());
    let baseline = *forward_distances
        .get(indices.get(&map.end).unwrap())
        .unwrap();
    let mut improvements = HashMap::new();

    // start of cheat
    for i1 in 0..map.height {
        for j1 in 0..map.width {
            if !indices.contains_key(&(i1, j1)) {
                continue;
            }
            for i2 in i1 - min(limit, i1)..i1 + limit + 1 {
                for j2 in j1 - min(limit, j1)..j1 + limit + 1 {
                    if !indices.contains_key(&(i2, j2)) {
                        continue;
                    }
                    let manhattan = i1.abs_diff(i2) + j1.abs_diff(j2);
                    if manhattan > limit {
                        continue;
                    }
                    let mut dist = *forward_distances
                        .get(indices.get(&(i1, j1)).unwrap())
                        .unwrap();
                    dist += manhattan;
                    dist += reverse_distances
                        .get(indices.get(&(i2, j2)).unwrap())
                        .unwrap();
                    if baseline > dist {
                        improvements.insert(((i1, j1), (i2, j2)), baseline - dist);
                    }
                }
            }
        }
    }

    improvements
}

pub fn count_improvements(improvements: HashMap<((usize, usize), (usize, usize)), usize>) -> u32 {
    let mut count = 0;
    for improvement in improvements {
        if improvement.1 >= 100 {
            count += 1;
        }
    }
    count
}
