
mod day12;
mod day13;
mod day19;

use day19::{read_input, count};
fn main() {
    let (words, patterns) = read_input();
    let count = count(&words, &patterns);
    dbg!(count);
}