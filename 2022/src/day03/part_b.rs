use anyhow::Result;
use itertools::Itertools;
use log::info;

pub fn run(text: &str) -> Result<()> {
    let result: u64 = text
        .split_terminator('\n')
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>())
        .map(|group| {
            let matching = group[0]
                .chars()
                .find(|c| group[1].chars().contains(&c) && group[2].chars().contains(&c))
                .unwrap();
            if matching.is_uppercase() {
                ((matching as u8) - ('A' as u8) + 27) as u64
            } else {
                ((matching as u8) - ('a' as u8) + 1) as u64
            }
        })
        .sum();
    info!("{result}");
    Ok(())
}
