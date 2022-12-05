use log::info;

pub fn run(text: &str) {
    let mut elves: Vec<u64> = text
        .split_terminator("\n\n")
        .map(|line| line.split('\n').filter_map(|s| s.parse::<u64>().ok()).sum())
        .collect();
    elves.sort_by(|a, b| b.cmp(a));
    let result: u64 = elves.iter().take(3).sum();
    info!("{result}");
}
