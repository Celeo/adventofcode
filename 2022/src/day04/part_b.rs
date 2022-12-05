use log::info;

pub fn run(text: &str) {
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
            let (a, b) = (group.first().unwrap(), group.get(1).unwrap());
            (a.0..=a.1).any(|i| (b.0..=b.1).contains(&i))
        })
        .count();

    info!("{result}");
}
