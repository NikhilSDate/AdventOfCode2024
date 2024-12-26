
mod day12;
mod day13;
mod day19;
mod day20;
mod day22;

use day22::{calc_sequence_prices, calc_sum, calculate_best_profit, read_input};
fn main() {
    let nums = read_input("data/day22/data.txt");
    let best_profit = calculate_best_profit(&nums);
    dbg!(best_profit);
}