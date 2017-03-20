extern crate clap;

use std::time::Duration;
use std::thread;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{Arg, App};


type Board = Vec<Vec<bool>>;

fn display_board(board: &Board) {

    let dead = ' ';
    let alive = 'o';

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

fn load_board(file: File) -> Board {

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut board: Board = Vec::new();
    let mut row = Vec::new();

    for c in contents.chars() {
        let c = match c {
            '0' => false,
            '1' => true,
            '\n' => {
                board.push(row);
                row = Vec::new();
                continue;
            },
            _ => panic!("The file content is invalid. It may contains only '0', '1' or '\\n'")
        };
        row.push(c);
    }

    if row.len() > 0 {
        board.push(row);
    }

    board
}

fn main() {
    let matches = App::new("Game of life")
                          .version("0.1")
                          .author("Antonio R. <antonioromerooca@gmail.com>")
                          .about("The \"game\" is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input.")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();

    let path = Path::new(matches.value_of("INPUT").unwrap());
    let display = path.display();

    let board_file = match File::open(path) {
            Err(_) => panic!("couldn't open {}.", display),
            Ok(file) => file,
    };

    let mut board = load_board(board_file);
    println!("{:?}", board);

    loop {
        print!("{}[2J", 27 as char);
        display_board(&board);
        transform_board(&mut board);
        thread::sleep(Duration::from_millis(200));
        print!("{}[2J", 27 as char);
    }
}
