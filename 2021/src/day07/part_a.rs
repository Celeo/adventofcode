fn move_to(positions: &[u64], to: i64) -> u64 {
    positions
        .iter()
        .map(|&n| (n as i64 - to).abs())
        .sum::<i64>() as u64
}

pub fn run() {
    let data = std::fs::read_to_string("src/day07/input.txt").unwrap();
    let positions: Vec<u64> = data.trim().split(',').map(|s| s.parse().unwrap()).collect();
    let mut least = (0, u64::MAX);
    for to in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() + 1 {
        let cost = move_to(&positions, to as i64);
        if cost < least.1 {
            least = (to, cost);
        }
    }
    println!("Result: {} (to {})", least.1, least.0);
}
