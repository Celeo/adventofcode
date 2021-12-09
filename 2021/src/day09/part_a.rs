const ROWS: usize = 100;
const COLUMNS: usize = 100;

pub fn run() {
    let raw = std::fs::read_to_string("src/day09/input.txt").unwrap();
    let mut area = [[0; COLUMNS]; ROWS];
    for (row, line) in raw.split_terminator('\n').enumerate() {
        for (column, c) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            area[row][column] = c;
        }
    }
    let mut lowest: Vec<u8> = Vec::new();

    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let mut compare_to: Vec<u8> = Vec::new();
            let level = area[row][column];
            if row > 0 {
                compare_to.push(area[row - 1][column]);
            }
            if row + 1 < ROWS {
                compare_to.push(area[row + 1][column]);
            }
            if column > 0 {
                compare_to.push(area[row][column - 1]);
            }
            if column + 1 < COLUMNS {
                compare_to.push(area[row][column + 1]);
            }
            if compare_to.iter().all(|&n| n > level) {
                lowest.push(level);
            }
        }
    }
    println!(
        "Result: {}",
        lowest.iter().map(|&n| n as u64 + 1u64).sum::<u64>()
    );
}
