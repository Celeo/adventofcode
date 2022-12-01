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
use log::debug;
use std::env;

/// Advent of Code 2022
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

    todo!()
}
