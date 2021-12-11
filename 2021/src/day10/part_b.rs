fn are_pair(a: char, b: char) -> bool {
    (a == '(' && b == ')')
        || (a == '[' && b == ']')
        || (a == '{' && b == '}')
        || (a == '<' && b == '>')
}

fn opposite(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn process_line(line: &str) -> Option<u64> {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                let last = stack.pop().unwrap();
                if !are_pair(last, c) {
                    return None;
                }
            }
        }
    }
    Some(
        stack
            .iter()
            .rev()
            .map(|&c| match opposite(c) {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            })
            .fold(0, |acc, n| 5 * acc + n),
    )
}

pub fn run() {
    let raw = std::fs::read_to_string("src/day10/input.txt").unwrap();
    let lines: Vec<_> = raw.split_terminator('\n').collect();
    let scores = {
        let mut scores = lines
            .iter()
            .filter_map(|&line| process_line(line))
            .collect::<Vec<_>>();
        scores.sort_unstable();
        scores
    };
    println!("Result: {}", scores.get(scores.len() / 2).unwrap()); // 3042730309
}
