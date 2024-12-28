use std::collections::HashSet;

use day23::{count_cycles_with_historian, get_next_Kn, get_three_cycles, read_input};
use petgraph::algo::connected_components;


mod day12;
mod day13;
mod day19;
mod day20;
mod day22;
mod day23;

fn main() {
    let g = read_input("data/day23/data.txt");
    let mut kn = HashSet::new();
    for edge in g.edge_indices() {
        let (a, b) = g.edge_endpoints(edge).unwrap();
        let mut subgraph = vec![a, b];
        subgraph.sort();
        kn.insert(subgraph);
    }
    for n in 3..14 {
        kn = get_next_Kn(&g, &kn);
        println!("K({}) = {}", n, kn.len());
    }
    let k13 = kn.iter().next().unwrap();
    let mut users: Vec<_> = k13.iter().map(|n| g.node_weight(*n).unwrap().clone()).collect();
    users.sort();
    dbg!(users.join(","));
}