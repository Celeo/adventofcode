use std::collections::HashSet;

type Point = (usize, usize);
type Fold = (char, usize);

fn parse(raw: &str) -> (HashSet<Point>, Vec<Fold>) {
    let lines: Vec<_> = raw.split('\n').collect();
    let split_at = lines.iter().position(|&line| line.is_empty()).unwrap();
    let dots: HashSet<_> = lines
        .iter()
        .take(split_at)
        .map(|&line| {
            let parts: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (
                parts.get(0).unwrap().to_owned(),
                parts.get(1).unwrap().to_owned(),
            )
        })
        .collect();
    let instructions: Vec<_> = lines
        .iter()
        .skip(split_at + 1)
        .take(lines.len() - split_at - 2)
        .map(|&line| {
            (
                line.chars().nth(11).unwrap(),
                line.chars()
                    .skip(13)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            )
        })
        .collect();
    (dots, instructions)
}

fn fold(dots: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut next = HashSet::new();
    for (dot_x, dot_y) in dots {
        if fold.0 == 'x' {
            if dot_x < &fold.1 {
                next.insert((*dot_x, *dot_y));
            } else {
                next.insert((2 * fold.1 - dot_x, *dot_y));
            }
        } else if dot_y < &fold.1 {
            next.insert((*dot_x, *dot_y));
        } else {
            next.insert((*dot_x, 2 * fold.1 - dot_y));
        }
    }
    next
}

pub fn run() {
    let raw = std::fs::read_to_string("src/day13/input.txt").unwrap();
    let (dots, instructions) = parse(&raw);
    let after_one = fold(&dots, instructions.get(0).unwrap());
    println!("Result: {}", after_one.len()); // 712
}
