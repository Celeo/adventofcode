pub fn run() {
    let spreadsheet = std::fs::read_to_string("src/day02/input.txt").unwrap();
    let mut diffs: Vec<usize> = Vec::new();
    for row in spreadsheet.split_terminator('\n') {
        let numbers = {
            let mut n = row
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            n.sort_unstable();
            n
        };
        diffs.push(numbers.iter().max().unwrap() - numbers.iter().min().unwrap());
    }
    log::info!("Result: {}", diffs.iter().sum::<usize>());
}
