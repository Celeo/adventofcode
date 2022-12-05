use log::info;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static WINS_AGAINST: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(3);
    let _ = map.insert('Y', 'A');
    let _ = map.insert('Z', 'B');
    let _ = map.insert('X', 'C');
    map
});

pub fn run(text: &str) {
    let rounds: Vec<usize> = text
        .split_terminator('\n')
        .map(|line| {
            let opp = line.chars().next().unwrap();
            let you = line.chars().nth(2).unwrap();
            let you_u = you as usize - 23;

            let win = WINS_AGAINST.get(&you).unwrap() == &opp;
            let compare = if win {
                6
            } else if you_u == (opp as usize) {
                3
            } else {
                0
            };

            compare + you_u - 64
        })
        .collect();
    info!("{}", rounds.iter().sum::<usize>());
}
