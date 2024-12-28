use std::{collections::HashMap, fs};
use std::cmp::max;

const M: u64 = 16777216;
type Sequence = (i64, i64, i64, i64);

pub fn read_input(path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(path).unwrap();
    let nums = contents
        .lines()
        .map(|line| str::parse(line).unwrap())
        .collect();
    nums
}

pub fn pseudorandom_update(num: u64) -> u64 {
    let mut result = num % M;
    result = (result * 64) ^ result;
    result = result % M;
    result = (result / 32) ^ result;
    result = result % M;
    result = (result * 2048) ^ result;
    result = result % M;
    result
}

pub fn sequence_push(sequence: &mut Sequence, diff: i64) {
    sequence.0 = sequence.1;
    sequence.1 = sequence.2;
    sequence.2 = sequence.3;
    sequence.3 = diff;
}

pub fn calc_pseudorandom(num: u64, iters: u64) -> u64 {
    let mut result = num % M;
    for _ in 0..iters {
        result = (result * 64) ^ result;
        result = result % M;
        result = (result / 32) ^ result;
        result = result % M;
        result = (result * 2048) ^ result;
        result = result % M;
    }
    result
}

pub fn calc_sequence_prices(num: u64, iters: u64) -> HashMap<Sequence, i64> {
    let mut sequence_prices = HashMap::new();
    let mut current = num;
    let mut sequence = (0, 0, 0, 0);
    for i in 0..iters {
        let next = pseudorandom_update(current);
        let current_price = (current % 10) as i64;
        let next_price = (next % 10) as i64;
        sequence_push(&mut sequence, next_price - current_price);
        current = next;
        if i >= 3 {
            if !sequence_prices.contains_key(&sequence) {
                sequence_prices.insert(sequence, next_price);
            }
        }
    }
    sequence_prices
}

pub fn calc_sum(nums: &Vec<u64>) -> u64 {
    let sum = nums.iter().map(|n| calc_pseudorandom(*n, 2000)).sum();
    sum
}

pub fn calculate_best_profit(nums: &Vec<u64>) -> i64 {
    let price_maps: Vec<_> = nums
        .iter()
        .map(|n| calc_sequence_prices(*n, 2000))
        .collect();
    let mut full_map = HashMap::new();
    for map in &price_maps {
        for k in map.keys() {
            if !full_map.contains_key(k) {
                full_map.insert(k, 0);
            }
            full_map.insert(k, full_map.get(k).unwrap() + map.get(k).unwrap());
        }
    }
    *full_map.values().max().unwrap()
}
