fn tick(fish: &[i8]) -> Vec<i8> {
    fish.iter()
        .flat_map(|f| if f - 1 == -1 { vec![6, 8] } else { vec![f - 1] })
        .collect()
}

pub fn run() {
    let data = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let mut fish: Vec<i8> = data.trim().split(',').map(|s| s.parse().unwrap()).collect();
    for _round in 0..80 {
        fish = tick(&fish);
    }
    println!("Result: {}", fish.len()); // 362639
}
