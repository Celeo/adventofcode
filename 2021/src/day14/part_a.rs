fn pass(template: &str, insertions: &[(char, char, char)]) -> String {
    let chars: Vec<_> = template.chars().collect();
    let mut ret: Vec<char> = Vec::new();

    for insertion in insertions {
        for index in 0..template.len() - 1 {
            if chars.get(index).unwrap() == &insertion.0
                && chars.get(index + 1).unwrap() == &insertion.1
            {
                //
            }
        }
    }

    ret.iter().collect()
}

pub fn run() {
    let raw = std::fs::read_to_string("src/day14/input.txt").unwrap();
    let lines: Vec<_> = raw.split_terminator('\n').collect();
    let template = lines.get(0).unwrap();
    let insertions: Vec<_> = lines
        .iter()
        .skip(2)
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line.chars().nth(1).unwrap(),
                line.chars().nth(6).unwrap(),
            )
        })
        .collect();

    dbg!(pass(template, &insertions));
}
