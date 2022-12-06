use std::collections::HashSet;

use log::info;

pub fn run(text: &str) {
    for index in 4..text.len() {
        if HashSet::<char>::from_iter(text[index - 4..index].chars().into_iter()).len() == 4 {
            info!("{index}");
            return;
        }
    }
}
