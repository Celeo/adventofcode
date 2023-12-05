use once_cell::sync::Lazy;
use regex::Regex;
use std::{cmp::max, collections::HashMap};

static CUBES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+ \w+").unwrap());

pub fn run(text: &str) {
    let val: u32 = text
        .split_terminator('\n')
        .map(|line| {
            let mut map: HashMap<&str, u32> = HashMap::new();
            for captures in CUBES.find_iter(line) {
                let mut parts = captures.as_str().split(' ');
                let count: u32 = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap();
                _ = map
                    .entry(color)
                    .and_modify(|c| *c = max(*c, count))
                    .or_insert(count);
            }
            let mut values = map.values();
            let start = values.next().unwrap();
            values.fold(*start, |acc, e| acc * e)
        })
        .sum();

    dbg!(val);
}
