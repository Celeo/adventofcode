use log::{debug, error};
use std::env;

mod day01;
mod day02;

const DAY_RESOLVER: &[(&str, fn())] = &[
    ("day01a", day01::part_a::run),
    ("day01b", day01::part_b::run),
    ("day02a", day02::part_a::run),
    ("day02b", day02::part_b::run),
];

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();
    debug!("Starting");

    let args: Vec<_> = env::args().skip(1).collect();
    if let Some(target_number) = args.get(0) {
        debug!("Target specified: {}", target_number);
        let day_fn = if target_number.starts_with("day") {
            target_number.to_owned()
        } else {
            format!("day{}", target_number)
        };
        debug!("Day function: {}", day_fn);
        if let Some(matching) = DAY_RESOLVER.iter().find(|&(name, _)| name == &day_fn) {
            debug!("Running {}", matching.0);

            matching.1();

            debug!("Done");
        } else {
            error!("Could not find matching function");
        }
    } else {
        error!("No target supplied");
    }
}
