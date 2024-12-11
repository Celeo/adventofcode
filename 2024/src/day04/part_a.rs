use log::info;
use std::cmp::min;

pub fn run(text: &str) {
    let lines: Vec<Vec<char>> = text
        .split_terminator('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for y in 0..lines.len() {
        let line = lines.get(y).unwrap();
        for x in 0..line.len() {
            // // good
            // let hor: String = line.iter().skip(x).take(4).collect();
            // if hor == "XMAS" || hor == "SAMX" {
            //     count += 1;
            // }

            // // good
            // let vert: String = (0..=3)
            //     .map(|add| {
            //         lines
            //             .get(min(lines.len() - 1, y + add))
            //             .unwrap()
            //             .get(x)
            //             .unwrap()
            //     })
            //     .collect();
            // if vert == "XMAS" || vert == "SAMX" {
            //     count += 1;
            // }

            // bad - this will ride the borders
            let diag: String = (0..=3)
                .map(|add| {
                    lines
                        .get(min(lines.len() - 1, y + add))
                        .unwrap()
                        .get(min(line.len() - 1, x + add))
                        .unwrap()
                })
                .collect();
            if diag == "XMAS" || diag == "SAMX" {
                count += 1;
            }
            info!("{y} {x}: {diag}");
        }
    }

    println!("\n\n");
    info!("{count}");
}
