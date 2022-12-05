use anyhow::Result;
use log::info;

pub fn run(text: &str) -> Result<()> {
    let result = text
        .split_terminator('\n')
        .map(|line| {
            line.split(',')
                .map(|group| {
                    group
                        .split_once('-')
                        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .filter(|group| {
            let (a, b) = (group.get(0).unwrap(), group.get(1).unwrap());
            (a.0 <= b.0 && a.1 >= b.1) || (a.0 >= b.0 && a.1 <= b.1)
        })
        .count();

    info!("{result}");
    Ok(())
}