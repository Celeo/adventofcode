use log::{debug, info};
use std::collections::HashMap;

pub fn run(text: &str) {
    let mut index = 1u32;
    let mut x = 1i32;
    let mut pending: HashMap<u32, i32> = HashMap::new();

    for line in text.split_terminator('\n') {
        if line.starts_with("addx ") {
            let val = line.chars().skip(5).collect::<String>().parse().unwrap();
            let _ = pending.insert(index + 3, val);
        }
        debug!("Index {index}, x = {x}");
        index += 1;
        if let Some(m) = pending.remove(&index) {
            x += m;
        }
        if index == 20 || index % 40 == 0 {
            info!(
                "Index {index}, x = {x}, signal = {}",
                i64::from(index) * i64::from(x)
            );
        }
    }
    // while !pending.is_empty() {
    //     debug!("Index {index}, x = {x}");
    //     index += 1;
    //     if let Some(m) = pending.remove(&index) {
    //         x += m;
    //     }
    //     if index == 20 || index % 40 == 0 {
    //         info!("Index {index}, x = {x}, signal = {}", (index as i32) * x);
    //     }
    // }

    debug!("End of tape, index {index}, x = {x}");
}
