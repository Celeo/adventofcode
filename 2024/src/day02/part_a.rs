use itertools::Itertools;
use log::info;
use std::collections::HashSet;

fn valid(line: &[i64]) -> bool {
    let as_set: HashSet<_> = line.iter().collect();
    if as_set.len() != line.len() {
        return false;
    }
    let sorted: Vec<_> = line.iter().sorted_unstable().copied().collect();
    if sorted != line {
        let sorted: Vec<_> = sorted.iter().rev().copied().collect();
        if sorted != line {
            return false;
        }
    }
    for i in 1..line.len() {
        let delta = (line[i] - line[i - 1]).abs();
        if delta > 3 {
            return false;
        }
    }
    true
}

pub fn run(text: &str) {
    let lines: Vec<Vec<i64>> = text
        .split_terminator('\n')
        .map(|line| line.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect();

    let count = lines.iter().filter(|line| valid(&line)).count();
    info!("{count}");
}
