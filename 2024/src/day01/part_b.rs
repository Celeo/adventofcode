#![allow(clippy::cast_possible_wrap)]

use itertools::Itertools;
use log::info;

pub fn run(text: &str) {
    let numbers: Vec<(i32, i32)> = text
        .split_terminator('\n')
        .map(|line| {
            let left = line
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
            let right = line
                .chars()
                .skip_while(|c| c.is_numeric())
                .skip_while(char::is_ascii_whitespace)
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
            (left, right)
        })
        .collect();

    let left: Vec<_> = numbers.iter().map(|(n, _)| n).sorted_unstable().collect();
    let right: Vec<_> = numbers.iter().map(|(_, n)| n).sorted_unstable().collect();

    let total = left.iter().fold(0, |acc, &l| {
        let occurrences = right.iter().filter(|&r| r == &l).count();
        acc + (l * occurrences as i32)
    });
    info!("{total}");
}
