use log::info;
use once_cell::sync::Lazy;

static NUMBERS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
});

fn take(line: &str) -> (&str, Option<u32>) {
    let c = line.chars().nth(0).unwrap();
    if c.is_digit(10) {
        return (&line[1..], Some(c.to_digit(10).unwrap()));
    }
    for (num, number) in NUMBERS.iter().enumerate() {
        if line.starts_with(number) {
            return (&line[number.len() - 1..], Some(num as u32 + 1));
        }
    }

    (&line[1..], None)
}

fn process_line(line: &str) -> Vec<u32> {
    let mut line = line;
    let mut vals = Vec::new();
    while !line.is_empty() {
        let (rest, val) = take(line);
        if let Some(v) = val {
            vals.push(v);
        }
        line = rest;
    }
    vals
}

pub fn run(text: &str) {
    let val: u32 = text
        .split_terminator('\n')
        .map(|line| {
            let nums = process_line(line);
            10 * nums.first().unwrap() + nums.last().unwrap()
        })
        .sum();

    info!("{val}");
}
