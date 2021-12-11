fn are_pair(a: char, b: char) -> bool {
    (a == '(' && b == ')')
        || (a == '[' && b == ']')
        || (a == '{' && b == '}')
        || (a == '<' && b == '>')
}

fn score_line(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                let last = stack.pop().unwrap();
                if !are_pair(last, c) {
                    return match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                }
            }
        }
    }
    0
}

pub fn run() {
    let raw = std::fs::read_to_string("src/day10/input.txt").unwrap();
    let lines: Vec<_> = raw.split_terminator('\n').collect();
    let result: u64 = lines.iter().map(|&line| score_line(line)).sum();
    println!("Result: {}", result);
}
