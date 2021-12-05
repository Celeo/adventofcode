#![allow(unused)]

use std::collections::HashMap;

fn parse_line(line: &str) -> (usize, usize, usize, usize) {
    let parts: Vec<_> = line.split(&[',', ' '][..]).collect();
    let numbers: Vec<usize> = parts
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 2)
        .map(|(_, s)| s.parse().unwrap())
        .collect();
    (
        *numbers.get(0).unwrap(),
        *numbers.get(1).unwrap(),
        *numbers.get(2).unwrap(),
        *numbers.get(3).unwrap(),
    )
}

fn build_map() -> HashMap<(usize, usize), usize> {
    let data = std::fs::read_to_string("src/day05/input.txt").unwrap();
    let mut map = HashMap::new();
    for line in data.split_terminator('\n') {
        let (x1, y1, x2, y2) = parse_line(line);
        if x1 != x2 && y1 != y2 {
            continue;
        }
        // FIXME this won't work; (x2, y2) may be less than (x1, y1)
        for x_mod in x1..x2 + 1 {
            for y_mod in y1..y2 + 1 {
                //
            }
        }
    }
    map
}

pub fn run() {
    let map = build_map();

    //
}
