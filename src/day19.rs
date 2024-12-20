use std::collections::HashSet;
use std::fs;

pub fn read_input() -> (Vec<String>, HashSet<String>) {
    let input = fs::read_to_string("data/day19/data.txt").unwrap();
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let patterns = first.split(",");
    let mut dict = HashSet::new();
    patterns.for_each(|x| { dict.insert(x.trim().to_string()); });
    lines.next();
    let mut words = Vec::new();
    for item in lines {
        words.push(item.trim().to_string());
    }
    (words, dict)
}

fn possible(word: &str, patterns: &HashSet<String>) -> u64 {
    let mut dp = vec![0 as u64; word.len() + 1];
    dp[word.len()] = 1;

    for i in (0..word.len()).rev() {
        for j in ((i + 1)..(word.len() + 1)) {
            if patterns.contains(&word[i..j]) && dp[j] > 0 {
                dp[i] += dp[j];
            }
        }
    }
    dp[0]
}

pub fn count(words: &Vec<String>, patterns: &HashSet<String>) -> u64 {
    let mut count = 0;
    for word in words {
        count += possible(word, patterns);
    }
    count
}