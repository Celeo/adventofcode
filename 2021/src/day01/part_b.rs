pub fn run() {
    let values: Vec<_> = std::fs::read_to_string("src/day01/input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let result = values
        .iter()
        .take(values.len() - 2)
        .enumerate()
        .map(|(i, v)| v + values.get(i + 1).unwrap() + values.get(i + 2).unwrap())
        .fold((usize::MAX, 0), |acc, value| {
            (value, if value > acc.0 { acc.1 + 1 } else { acc.1 })
        })
        .1;
    println!("Result: {}", result); // 1633
}
