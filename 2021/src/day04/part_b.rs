/* SOLUTION DOESN'T WORK YET */

type Board = [[usize; 5]; 5];

fn load_board(lines: &[&str]) -> Board {
    let mut board = [[0; 5]; 5];
    for (index, row) in lines.iter().enumerate() {
        let d: Vec<usize> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
        board[index] = d.try_into().unwrap();
    }
    board
}

fn load_data() -> (Vec<usize>, Vec<Board>) {
    let raw = std::fs::read_to_string("src/day04/input.txt").unwrap();
    let lines: Vec<_> = raw.split_terminator('\n').chain([""]).collect();
    let draws: Vec<_> = lines
        .get(0)
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut board_buffer = Vec::new();
    for &line in lines.iter().skip(2) {
        if line.is_empty() {
            boards.push(load_board(&board_buffer));
            board_buffer.clear();
            continue;
        }
        board_buffer.push(line);
    }
    (draws, boards)
}

fn check_board(board: &Board, drawn: &[usize]) -> bool {
    for index in 0..5 {
        // check rows
        let mut win = true;
        for number in board[index] {
            if !drawn.contains(&number) {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
        // check columns
        win = true;
        for number in board.iter().map(|row| row[index]) {
            if !drawn.contains(&number) {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    false
}

fn check_win(boards: &[Board], drawn: &[usize]) -> Option<(usize, usize)> {
    for (index, board) in boards.iter().enumerate() {
        if check_board(board, drawn) {
            let numbers = board
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|n| !drawn.contains(n))
                        .copied()
                        .collect::<Vec<usize>>()
                })
                .reduce(|mut a, mut b| {
                    a.append(&mut b);
                    a
                })
                .unwrap();
            return Some((
                index,
                numbers.iter().sum::<usize>() * drawn[drawn.len() - 1],
            ));
        }
    }
    None
}

pub fn run() {
    let (draws, mut boards) = load_data();
    for draw_index in 5..draws.len() {
        let drawn: Vec<_> = draws.iter().take(draw_index).copied().collect();
        if let Some((index, result)) = check_win(&boards, &drawn) {
            boards.remove(index);
            // FIXME this isn't being reached
            if boards.is_empty() {
                println!("Result: {}", result);
                return;
            }
        }
    }
}

/* ANSWER IS 16830 */
