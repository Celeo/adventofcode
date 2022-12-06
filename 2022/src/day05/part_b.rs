#![allow(clippy::needless_for_each)]

use itertools::Itertools;
use log::info;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static INSTRUCTION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap());

pub fn run(text: &str) {
    let mut boxes: HashMap<usize, Vec<char>> = HashMap::new();

    text.split_terminator('\n').for_each(|line| {
        if !line.is_empty() {
            if let Some(captures) = INSTRUCTION_REGEX.captures(line) {
                let (count, from, to) = (
                    captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                );
                let mut removed: Vec<_> = (0..count)
                    .map(|_| boxes.get_mut(&from).unwrap().pop().unwrap())
                    .collect();
                removed.reverse();
                removed.iter().for_each(|&b| {
                    boxes.get_mut(&to).unwrap().push(b);
                });
            } else {
                let columns: Vec<_> = line
                    .chars()
                    .chunks(4)
                    .into_iter()
                    .map(std::iter::Iterator::collect::<String>)
                    .collect();
                columns.iter().enumerate().for_each(|(index, item)| {
                    let index = index + 1;
                    let _ = boxes.entry(index).or_insert_with(Vec::new);
                    if !item.trim().is_empty() {
                        boxes
                            .get_mut(&index)
                            .unwrap()
                            .insert(0, item.chars().nth(1).unwrap());
                    }
                });
            }
        }
    });

    let mut result = String::new();
    for index in 1.. {
        if !boxes.contains_key(&index) {
            break;
        }
        result.push(*boxes.get(&index).unwrap().last().unwrap());
    }
    info!("{result}");
}
