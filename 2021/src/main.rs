mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

const DAY_RESOLVER: &[(&str, fn())] = &[
    ("day01a", day01::part_a::run),
    ("day01b", day01::part_b::run),
    ("day02a", day02::part_a::run),
    ("day02b", day02::part_b::run),
    ("day03a", day03::part_a::run),
    ("day03b", day03::part_b::run),
    ("day04a", day04::part_a::run),
    ("day04b", day04::part_b::run),
    ("day05a", day05::part_a::run),
    ("day05b", day05::part_b::run),
    ("day06a", day06::part_a::run),
    ("day06b", day06::part_b::run),
    ("day07a", day07::part_a::run),
    ("day07b", day07::part_b::run),
    ("day08a", day08::part_a::run),
    ("day08b", day08::part_b::run),
    ("day09a", day09::part_a::run),
    ("day09b", day09::part_b::run),
];

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let Some(target_number) = args.get(0) {
        let day_fn = if target_number.starts_with("day") {
            target_number.to_owned()
        } else {
            format!("day{}", target_number)
        };
        if let Some(matching) = DAY_RESOLVER.iter().find(|&(name, _)| name == &day_fn) {
            println!("Running {}", matching.0);
            matching.1();
        } else {
            eprintln!("Could not find matching function");
        }
    } else {
        eprintln!("No target supplied");
    }
}
