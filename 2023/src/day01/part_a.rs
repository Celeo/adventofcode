use log::info;

pub fn run(text: &str) {
    let val: u32 = text
        .split_terminator('\n')
        .map(|line| {
            let nums: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * nums.first().unwrap() + nums.last().unwrap()
        })
        .sum();

    info!("{val}");
}
