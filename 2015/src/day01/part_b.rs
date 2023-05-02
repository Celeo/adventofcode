use log::info;

pub fn run(text: &str) {
    let mut pos: i32 = 0;
    let mut rolling: i32 = 0;
    for c in text.chars().map(|c| if c == '(' { 1 } else { -1 }) {
        rolling += c;
        pos += 1;
        if rolling == -1 {
            break;
        }
    }
    info!("{pos}");
}
