use anyhow::{anyhow, Result};
use log::info;

pub fn run() -> Result<()> {
    let text = std::fs::read_to_string("src/day01/input.txt")?;
    let result: u64 = text
        .split("\n\n")
        .map(|line| line.split("\n").filter_map(|s| s.parse::<u64>().ok()).sum())
        .max()
        .ok_or(anyhow!("Could not find sum"))?;
    info!("{result}");
    Ok(())
}
