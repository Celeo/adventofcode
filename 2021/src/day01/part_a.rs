pub fn run() {
    let larger = std::fs::read_to_string("src/day01/input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|l| l.parse::<usize>().unwrap())
        .fold((usize::MAX, 0), |acc, value| {
            (value, if value > acc.0 { acc.1 + 1 } else { acc.1 })
        })
        .1;
    println!("Result: {}", larger); // 1602
}
