
mod day12;
mod day13;
mod day19;
mod day20;

use day20::{read_input, build_graph};
fn main() {
    let chars = read_input();
    let map = build_graph(chars);
    dbg!(map);
}