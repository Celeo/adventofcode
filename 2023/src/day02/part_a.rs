use log::info;
use once_cell::sync::Lazy;
use regex::Regex;

static GAME: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Game (\d+):"#).unwrap());
static CUBES: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\d+ \w+"#).unwrap());

pub fn run(text: &str) {
    let val: u32 = text
        .split_terminator('\n')
        .map(|line| {
            for captures in CUBES.find_iter(line) {
                let mut parts = captures.as_str().split(' ');
                let count: u32 = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap();
                let limit = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => unreachable!(),
                };
                if count > limit {
                    return 0;
                }
            }
            GAME.captures(line).unwrap()[1].parse().unwrap()
        })
        .sum();

    info!("{val}");
}
