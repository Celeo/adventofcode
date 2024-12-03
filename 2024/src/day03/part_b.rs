use log::info;
use regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|do\(\)").unwrap());

pub fn run(text: &str) {
    let mut enabled = true;
    let value = PATTERN.captures_iter(text).fold(0, |acc, cap| {
        let whole = cap.get(0).unwrap().as_str();
        match whole {
            "don't()" => {
                enabled = false;
                acc
            }
            "do()" => {
                enabled = true;
                acc
            }
            _ => {
                if enabled {
                    let left: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
                    let right: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
                    acc + left * right
                } else {
                    acc
                }
            }
        }
    });
    info!("{value}");
}
