use std::collections::HashSet;

use log::info;

pub fn run(text: &str) {
    for index in 14..text.len() {
        if text[index - 14..index]
            .chars()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            == 14
        {
            info!("{index}");
            return;
        }
    }
}
