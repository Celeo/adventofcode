fn tick(fish: &mut [usize; 9]) {
    let add = fish[0];
    for i in 1..9 {
        fish[i - 1] = fish[i];
    }
    fish[8] = add;
    fish[6] += add;
}

pub fn run() {
    let data = std::fs::read_to_string("src/day06/input.txt").unwrap();
    let mut fish = [0; 9];
    for f in data.trim().split(',') {
        fish[f.parse::<usize>().unwrap()] += 1;
    }
    for _round in 0..256 {
        tick(&mut fish);
    }
    println!("Result: {}", fish.iter().sum::<usize>()); // 1639854996917
}
