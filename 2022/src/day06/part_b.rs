use std::collections::HashSet;

use log::info;

pub fn run(text: &str) {
    for index in 14..text.len() {
        if HashSet::<char>::from_iter(text[index - 14..index].chars().into_iter()).len() == 14 {
            info!("{index}");
            return;
        }
    }
}
