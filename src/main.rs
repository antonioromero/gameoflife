type Board = Vec<Vec<bool>>;

fn display_board(board: &Board) {

    let dead = '░';
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
    neighbours.push(board[coord.0 - 1][coord.0 - 1]);
    neighbours.push(board[coord.0][coord.0 - 1]);
    neighbours.push(board[coord.0 + 1][coord.0 - 1]);

    neighbours.push(board[coord.0 - 1][coord.1]);
    neighbours.push(board[coord.0 + 1][coord.1]);

    neighbours.push(board[coord.0 - 1][coord.0 + 1]);
    neighbours.push(board[coord.0][coord.0 + 1]);
    neighbours.push(board[coord.0 + 1][coord.0 + 1]);

    let alive_count = neighbours.into_iter().filter(|i| *i == true).collect::<Vec<bool>>();
    alive_count.len() as u8
}

fn main() {

    let mut board = vec![
        vec![false, false, false, false, false],
        vec![false, false, false, false, false],
        vec![false, false, false, false, false],
        vec![false, false, false, false, false],
        vec![false, false, false, false, false],
    ];

    board[0][1] = true;
    board[1][1] = true;
    board[1][0] = true;

    println!("{:?}", count_neighbours(&board, (1,1)));
    //println!("{:#?}", board);
    display_board(&board);
}
