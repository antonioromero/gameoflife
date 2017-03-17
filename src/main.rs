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

    match count_cell_neighbours(board, coord) {
        3 if !board[coord.0][coord.1] => true,
        //0 | 1 => false,
        2 | 3 => true,
        _ => false,
    }
}

fn count_cell_neighbours(board: &Board, coord: (usize, usize)) -> u8 {

    let mut neighbours = Vec::new();
    let c = (coord.0 as i64, coord.1 as i64);

    let candidates = vec![
        (c.0 - 1, c.1 - 1),
        (c.0 - 1, c.1 + 1),
        (c.0 - 1, c.1),
        (c.0 + 1, c.1 - 1),
        (c.0 + 1, c.1 + 1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
    ];

    for c in candidates {
        let c0 = c.0 as usize;
        let c1 = c.1 as usize;

        let row = board.get(c0);
        if !row.is_some() {
            continue;
        }

        let cell = board[c0].get(c1);
        if !cell.is_some() {
            continue;
        }

        neighbours.push(board[c0][c1]);
    }

    let alive_count = neighbours.into_iter().filter(|i| *i == true).collect::<Vec<bool>>();
    alive_count.len() as u8
}

fn transform_board(board: &mut Board) {

    let mut new_board = board.clone();

    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            new_board[i][j] = cell_transformation(board, (i, j));
        }
    }

    *board = new_board
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
        transform_board(&mut board);
        thread::sleep(Duration::from_millis(200));
        print!("{}[2J", 27 as char);
    }
}
