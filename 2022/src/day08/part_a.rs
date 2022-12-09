use log::info;

pub fn run(text: &str) {
    let map: Vec<Vec<_>> = text
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible = map.get(0).unwrap().len() * 2 + (map.len() - 2) * 2;
    for y in 1..map.len() - 1 {
        for x in 1..map.get(y).unwrap().len() - 1 {
            let item = map.get(y).unwrap().get(x).unwrap();

            let row_max = map.get(y).unwrap().iter().max().unwrap();
            if item >= row_max {
                continue;
            }

            let column_max = map.iter().map(|row| row.get(x).unwrap()).max().unwrap();
            if item >= column_max {
                continue;
            }

            visible += 1;
        }
    }

    info!("{visible}"); // 8761, 8753 -- too high

    todo!()
}
