//! Advent of Code 2023

#![deny(
    clippy::all,
    clippy::pedantic,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use clap::Parser;
use log::{debug, error};
use std::{env, fs, process};

mod day01;
mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
// mod day26;
// mod day27;
// mod day28;
// mod day29;
// mod day30;
// mod day31;

#[allow(clippy::type_complexity)]
const DAY_RESOLVER: &[(&str, fn(&str))] = &[
    ("day01a", day01::part_a::run),
    ("day01b", day01::part_b::run),
    ("day02a", day02::part_a::run),
    ("day02b", day02::part_b::run),
    //     ("day03a", day03::part_a::run),
    //     ("day03b", day03::part_b::run),
    //     ("day04a", day04::part_a::run),
    //     ("day04b", day04::part_b::run),
    //     ("day05a", day05::part_a::run),
    //     ("day05b", day05::part_b::run),
    //     ("day06a", day06::part_a::run),
    //     ("day06b", day06::part_b::run),
    //     ("day07a", day07::part_a::run),
    //     ("day07b", day07::part_b::run),
    //     ("day08a", day08::part_a::run),
    //     ("day08b", day08::part_b::run),
    //     ("day09a", day09::part_a::run),
    //     ("day09b", day09::part_b::run),
    //     ("day10a", day10::part_a::run),
    //     ("day10b", day10::part_b::run),
    //     ("day11a", day11::part_a::run),
    //     ("day11b", day11::part_b::run),
    //     ("day12a", day12::part_a::run),
    //     ("day12b", day12::part_b::run),
    //     ("day13a", day13::part_a::run),
    //     ("day13b", day13::part_b::run),
    //     ("day14a", day14::part_a::run),
    //     ("day14b", day14::part_b::run),
    //     ("day15a", day15::part_a::run),
    //     ("day15b", day15::part_b::run),
    //     ("day16a", day16::part_a::run),
    //     ("day16b", day16::part_b::run),
    //     ("day17a", day17::part_a::run),
    //     ("day17b", day17::part_b::run),
    //     ("day18a", day18::part_a::run),
    //     ("day18b", day18::part_b::run),
    //     ("day19a", day19::part_a::run),
    //     ("day19b", day19::part_b::run),
    //     ("day20a", day20::part_a::run),
    //     ("day20b", day20::part_b::run),
    //     ("day21a", day21::part_a::run),
    //     ("day21b", day21::part_b::run),
    //     ("day22a", day22::part_a::run),
    //     ("day22b", day22::part_b::run),
    //     ("day23a", day23::part_a::run),
    //     ("day23b", day23::part_b::run),
    //     ("day24a", day24::part_a::run),
    //     ("day24b", day24::part_b::run),
    //     ("day25a", day25::part_a::run),
    //     ("day25b", day25::part_b::run),
    //     ("day26a", day26::part_a::run),
    //     ("day26b", day26::part_b::run),
    //     ("day27a", day27::part_a::run),
    //     ("day27b", day27::part_b::run),
    //     ("day28a", day28::part_a::run),
    //     ("day28b", day28::part_b::run),
    //     ("day29a", day29::part_a::run),
    //     ("day29b", day29::part_b::run),
    //     ("day30a", day30::part_a::run),
    //     ("day30b", day30::part_b::run),
    //     ("day31a", day31::part_a::run),
    //     ("day31b", day31::part_b::run),
];

/// Advent of Code 2023
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day and part to run
    day_part: String,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    if args.debug {
        env::set_var("RUST_LOG", "debug");
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    debug!("CLI args: {:?}", args);

    let day_fn = if args.day_part.starts_with("day") {
        args.day_part
    } else {
        format!("day{}", args.day_part)
    };
    if let Some(matching) = DAY_RESOLVER.iter().find(|&(name, _)| name == &day_fn) {
        let path = format!("src/{}/input.txt", &day_fn[..day_fn.len() - 1]);
        debug!("Loading {path}");
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                error!("Could not read input file: {e}");
                process::exit(1);
            }
        };
        debug!("Running {}", matching.0);
        matching.1(&text);
    } else {
        error!("Could not find matching function");
    }
}
