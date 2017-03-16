use std::time::Duration;
use std::thread;

type Board = Vec<Vec<bool>>;

fn display_board(board: &Board) {

    let dead = ' ';
    let alive = '▓';

    for row in board {
        for cell in row {
            let cell_repr = match *cell {
                true => alive,
                false => dead
            };
            print!("{}", cell_repr);
        }
        println!("");
    }
}

fn cell_transformation(board: &Board, coord: (usize, usize)) -> bool {
    match count_neighbours(board, coord) {
        3 if !board[coord.0][coord.1] => true,
        //0 | 1 => false,
        2 | 3 => true,
        _ => false,
    }
}

fn count_neighbours(board: &Board, coord: (usize, usize)) -> u8 {

    let mut neighbours = Vec::new();

    if coord.0 > 0 {
        if coord.1 > 0 {
            neighbours.push(board[coord.0 - 1][coord.1 - 1]);
        }

        if coord.1 < (board[coord.0 - 1].len() - 1) {
            neighbours.push(board[coord.0 - 1][coord.1 + 1]);
        }

        neighbours.push(board[coord.0 - 1][coord.1]);
    }

    if coord.0 < (board.len() - 1) {
        if coord.1 > 0 {
            neighbours.push(board[coord.0 + 1][coord.1 - 1]);
        }

        if coord.1 < (board[coord.0 + 1].len() - 1) {
            neighbours.push(board[coord.0 + 1][coord.1 + 1]);
        }

        neighbours.push(board[coord.0 + 1][coord.1]);
    }

    if coord.1 > 0 {
        neighbours.push(board[coord.0][coord.1 - 1]);
    }

    if coord.1 < (board[coord.0].len() - 1) {
        neighbours.push(board[coord.0][coord.1 + 1]);
    }

    let alive_count = neighbours.into_iter().filter(|i| *i == true).collect::<Vec<bool>>();
    alive_count.len() as u8
}

fn check_cells_transformations(board: &Board) -> Board {

    let mut new_board = board.clone();
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            new_board[i][j] = cell_transformation(board, (i, j));
        }
    }

    new_board
}

fn main() {

    let mut board = vec![
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false],
    ];

    board[0][1] = true;
    board[1][1] = true;
    board[1][0] = true;

    loop {
        print!("{}[2J", 27 as char);
        display_board(&board);
        board = check_cells_transformations(&board);
        thread::sleep(Duration::from_millis(200));
        print!("{}[2J", 27 as char);
    }
}
