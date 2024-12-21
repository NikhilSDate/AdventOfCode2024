
mod day12;
mod day13;
mod day19;
mod day20;

use day20::{build_graph, calculate_improvements, read_input};
fn main() {
    let chars = read_input();
    let map = build_graph(chars);
    calculate_improvements(&map);
}