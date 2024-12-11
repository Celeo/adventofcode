pub fn run(text: &str) {
    let rules: Vec<_> = text
        .split_terminator('\n')
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<u32> = line.split('|').map(|s| s.parse().unwrap()).collect();
            (*parts.get(0).unwrap(), *parts.get(1).unwrap())
        })
        .collect();

    let updates: Vec<Vec<u32>> = text
        .split_terminator('\n')
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    for update in updates {
        for n in update {
            //
        }
    }
}
