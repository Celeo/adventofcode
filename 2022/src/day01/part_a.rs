use log::info;

pub fn run(text: &str) {
    let result: u64 = text
        .split_terminator("\n\n")
        .map(|line| line.split('\n').filter_map(|s| s.parse::<u64>().ok()).sum())
        .max()
        .unwrap();
    info!("{result}");
}
