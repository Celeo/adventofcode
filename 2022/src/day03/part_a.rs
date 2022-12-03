use anyhow::Result;
use log::info;

pub fn run(text: &str) -> Result<()> {
    let result: u64 = text
        .split_terminator('\n')
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let matching = a.chars().find(|c| b.contains(&c.to_string())).unwrap();
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
