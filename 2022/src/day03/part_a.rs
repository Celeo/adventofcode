use log::info;

pub fn run(text: &str) {
    let result: u64 = text
        .split_terminator('\n')
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let matching = a.chars().find(|c| b.contains(&c.to_string())).unwrap();
            if matching.is_uppercase() {
                u64::from((matching as u8) - b'A' + 27)
            } else {
                u64::from((matching as u8) - b'a' + 1)
            }
        })
        .sum();
    info!("{result}");
}
