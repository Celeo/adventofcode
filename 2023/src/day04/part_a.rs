use log::info;
use std::collections::HashSet;

pub fn run(text: &str) {
    let val: u32 = text
        .split_terminator('\n')
        .map(|line| {
            let nums = line.split(": ").skip(1).next().unwrap();
            let winning: HashSet<u32> = nums
                .split(" | ")
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let choices: HashSet<u32> = nums
                .split(" | ")
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let overlap: HashSet<&u32> = winning.intersection(&choices).collect();
            if overlap.len() > 0 {
                (2u32).pow(overlap.len() as u32 - 1)
            } else {
                0
            }
        })
        .sum();

    info!("{val}");
}
