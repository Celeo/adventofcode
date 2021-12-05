pub fn run() {
    let data = std::fs::read_to_string("src/day03/input.txt").unwrap();
    let lines: Vec<_> = data.split_terminator('\n').collect();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for column in 0..lines.first().unwrap().len() {
        let chars: Vec<_> = lines
            .iter()
            .map(|line| line.chars().nth(column).unwrap())
            .collect();
        let zero_more = chars.iter().filter(|&&c| c == '0').count()
            > chars.iter().filter(|&&c| c == '1').count();
        gamma.push(if zero_more { '0' } else { '1' });
        epsilon.push(if zero_more { '1' } else { '0' });
    }
    println!(
        "Result: {}",
        usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
    ); // 775304
}
