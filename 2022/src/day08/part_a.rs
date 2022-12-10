use log::info;

pub fn run(text: &str) {
    let map: Vec<Vec<_>> = text
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = map.len();
    let columns = map.get(0).unwrap().len();
    let mut visible = columns * 2 + (rows - 2) * 2;

    for y in 1..rows - 1 {
        for x in 1..columns - 1 {
            let row = map.get(y).unwrap();
            let column: Vec<u32> = map.iter().map(|row| *row.get(x).unwrap()).collect();
            let val = map.get(y).unwrap().get(x).unwrap();

            if (0..x).map(|xi| row.get(xi).unwrap()).max().unwrap() < val
                || (x + 1..columns)
                    .map(|xi| row.get(xi).unwrap())
                    .max()
                    .unwrap()
                    < val
                || (0..y).map(|yi| column.get(yi).unwrap()).max().unwrap() < val
                || (y + 1..rows)
                    .map(|yi| column.get(yi).unwrap())
                    .max()
                    .unwrap()
                    < val
            {
                info!("({x}, {y}) == {val} is visible");
                visible += 1;
                continue;
            }
        }
    }
    info!("{visible}");
}
