use std::collections::HashSet;

const ROWS: usize = 10;
const COLUMNS: usize = 10;
const OFFSETS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

type Grid = [[u32; ROWS]; COLUMNS];

fn load_grid(raw: &str) -> Grid {
    let mut grid = [[0u32; ROWS]; COLUMNS];
    raw.split_terminator('\n')
        .enumerate()
        .for_each(|(row_index, row)| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .for_each(|(column_index, value)| {
                    grid[row_index][column_index] = value;
                })
        });
    grid
}

fn flash(grid: &mut Grid) -> usize {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    (0..10).for_each(|row| {
        (0..10).for_each(|column| {
            grid[row][column] += 1;
        })
    });
    loop {
        let mut dirty = false;
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if grid[row][column] > 9 && !flashed.contains(&(row, column)) {
                    flashed.insert((row, column));
                    dirty = true;
                    for (row_diff, column_diff) in OFFSETS {
                        let row_new = row as isize + row_diff;
                        let column_new = column as isize + column_diff;
                        if (0..10).contains(&row_new) && (0..10).contains(&column_new) {
                            grid[row_new as usize][column_new as usize] += 1;
                        }
                    }
                }
            }
        }
        if !dirty {
            break;
        }
    }
    flashed
        .iter()
        .for_each(|&(row, column)| grid[row][column] = 0);
    flashed.len()
}

pub fn run() {
    let raw = std::fs::read_to_string("src/day11/input.txt").unwrap();
    let mut grid = load_grid(&raw);
    let mut flashed = 0;
    for _ in 0..100 {
        flashed += flash(&mut grid);
    }
    println!("Result: {}", flashed); // 1603
}
