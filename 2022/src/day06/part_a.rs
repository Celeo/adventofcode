use std::collections::HashSet;

use log::info;

pub fn run(text: &str) {
    for index in 4..text.len() {
        if text[index - 4..index]
            .chars()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            == 4
        {
            info!("{index}");
            return;
        }
    }
}
