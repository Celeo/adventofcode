use log::info;
use std::fs;

pub fn parse() -> Vec<u32> {
    let values = fs::read_to_string("src/day01/input.txt").unwrap();
    values
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chain([values.chars().next().unwrap().to_digit(10).unwrap()])
        .collect()
}

pub fn run() {
    let numbers = parse();
    let mut sum = 0;
    let mut last = numbers.first().unwrap();
    for number in numbers.iter().skip(1) {
        if number == last {
            sum += number;
        }
        last = number;
    }
    info!("Result is {}", sum);
}
