fn most_in_column(lines: &[&str], column: usize) -> char {
    let chars: Vec<_> = lines
        .iter()
        .map(|&line| line.chars().nth(column).unwrap())
        .collect();
    if chars.iter().filter(|&&c| c == '0').count() > chars.iter().filter(|&&c| c == '1').count() {
        '0'
    } else {
        '1'
    }
}

fn find_rating(mut lines: Vec<&str>, most_common: bool) -> usize {
    let columns = lines.first().unwrap().len();
    loop {
        for column in 0..columns {
            if lines.len() == 1 {
                return usize::from_str_radix(lines.get(0).unwrap(), 2).unwrap();
            }
            let most = most_in_column(&lines, column);
            let keep = if most_common {
                most
            } else if most == '0' {
                '1'
            } else {
                '0'
            };
            lines.retain(|&e| e.chars().nth(column).unwrap() == keep);
        }
    }
}

pub fn run() {
    let data = std::fs::read_to_string("src/day03/input.txt").unwrap();
    let lines: Vec<_> = data.split_terminator('\n').collect();
    log::info!(
        "Result: {}",
        find_rating(lines.clone(), true) * find_rating(lines.clone(), false)
    ); // 1370737
}
