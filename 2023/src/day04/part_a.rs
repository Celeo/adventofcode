use log::info;
use std::collections::HashSet;

pub fn run(text: &str) {
    let val: u32 = text
        .split_terminator('\n')
        .map(|line| {
            let mut nums = line.split(": ").nth(1).unwrap().split(" | ");
            let winning: HashSet<u32> = nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let choices: HashSet<u32> = nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let overlap: HashSet<&u32> = winning.intersection(&choices).collect();
            if overlap.is_empty() {
                2u32.pow(overlap.len() as u32 - 1)
            } else {
                0
            }
        })
        .sum();

    info!("{val}");
}
