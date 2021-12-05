pub fn run() {
    let (x, y, _aim) = std::fs::read_to_string("src/day02/input.txt")
        .unwrap()
        .split_terminator('\n')
        .fold((0, 0, 0), |(x, y, aim), line| {
            let mut parts = line.split_whitespace();
            let action = parts.next().unwrap();
            let amount: usize = parts.next().unwrap().parse().unwrap();
            match action {
                "forward" => (x + amount, y + (amount * aim), aim),
                "down" => (x, y, aim + amount),
                "up" => (x, y, aim - amount),
                _ => (x, y, aim),
            }
        });
    println!("Result: {}", x * y); // 1845455714
}
