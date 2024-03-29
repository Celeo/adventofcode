use log::info;

pub fn run(text: &str) {
    let result: usize = text
        .split_terminator('\n')
        .map(|line| {
            let opp = line.chars().next().unwrap();

            let (you, compare) = match line.chars().nth(2).unwrap() {
                'X' => (
                    match opp {
                        'A' => 'C',
                        'B' => 'A',
                        'C' => 'B',
                        _ => panic!(),
                    },
                    0,
                ),
                'Y' => (opp, 3),
                'Z' => (
                    match opp {
                        'A' => 'B',
                        'B' => 'C',
                        'C' => 'A',
                        _ => panic!(),
                    },
                    6,
                ),
                _ => panic!(),
            };

            compare + (you as usize - 64)
        })
        .sum();

    info!("{}", result);
}
