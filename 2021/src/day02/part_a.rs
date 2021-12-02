pub fn run() {
    let (x, y) = std::fs::read_to_string("src/day02/input.txt")
        .unwrap()
        .split_terminator('\n')
        .fold((0, 0), |(x, y), line| {
            let mut parts = line.split_whitespace();
            let action = parts.next().unwrap();
            let amount: usize = parts.next().unwrap().parse().unwrap();
            match action {
                "forward" => (x + amount, y),
                "down" => (x, y + amount),
                "up" => (x, y - amount),
                _ => (x, y),
            }
        });
    log::info!("Result: {}", x * y); // 1855814
}
