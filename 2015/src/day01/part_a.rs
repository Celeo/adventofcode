use log::info;

pub fn run(text: &str) {
    let result: i32 = text.chars().map(|c| if c == '(' { 1 } else { -1 }).sum();
    info!("{result}");
}
