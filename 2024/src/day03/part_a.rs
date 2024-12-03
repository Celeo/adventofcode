use std::sync::LazyLock;

use log::info;
use regex::Regex;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

pub fn run(text: &str) {
    let value = PATTERN.captures_iter(text).fold(0, |acc, cap| {
        let left: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let right: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
        acc + left * right
    });
    info!("{value}");
}
