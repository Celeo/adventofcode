use itertools::Itertools;
use log::info;

pub fn run(text: &str) {
    let result: u64 = text
        .split_terminator('\n')
        .chunks(3)
        .into_iter()
        .map(std::iter::Iterator::collect::<Vec<_>>)
        .map(|group| {
            let matching = group[0]
                .chars()
                .find(|c| group[1].chars().contains(c) && group[2].chars().contains(c))
                .unwrap();
            if matching.is_uppercase() {
                u64::from((matching as u8) - b'A' + 27)
            } else {
                u64::from((matching as u8) - b'a' + 1)
            }
        })
        .sum();
    info!("{result}");
}
