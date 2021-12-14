use std::fs;
use std::io::{self, BufRead};

pub fn day_four() {
    let file = fs::File::open("resources/04/sample.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let number_sequence = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::<(Vec<Vec<(u32, bool)>>, bool)>::new();

    for line in lines {
        if line.as_ref().unwrap() != "" {
            let board_line = line
                .unwrap()
                .trim()
                .split(" ")
                .filter(|&item| item != "")
                .map(|item| (item.parse::<u32>().unwrap(), false))
                .collect::<Vec<(u32, bool)>>();
            boards.last_mut().unwrap().0.push(board_line);
        } else {
            boards.push((Vec::<Vec<(u32, bool)>>::new(), false));
        }
    }

    println!("Part 1");
    compute_winning_board(number_sequence, boards);
}

fn compute_winning_board(
    number_sequence: Vec<u32>,
    mut boards: Vec<(Vec<Vec<(u32, bool)>>, bool)>,
) {
    for (i_seq, n) in number_sequence.iter().enumerate() {
        for (i_board, board) in boards.iter_mut().enumerate() {
            let mut co = Vec::new();
            for (i, row) in board.0.iter_mut().enumerate() {
                for (j, (number, is_checked)) in row.iter_mut().enumerate() {
                    if number == n {
                        *is_checked = true;
                        co.push((i, j))
                    }
                }
            }

            let mut board_wins = false;
            for (x, y) in co {
                let mut row_full = true;
                let mut column_full = true;
                for z in 0..5 {
                    row_full = row_full && board.0[x][z].1;
                    column_full = column_full && board.0[z][y].1;
                }
                if row_full || column_full {
                    board_wins = true;
                }
            }

            if board_wins && !board.1 {
                let sum_unmarked = board
                    .0
                    .iter()
                    .flatten()
                    .filter(|&item| !item.1)
                    .cloned()
                    .reduce(|a, b| (a.0 + b.0, b.1 || a.1))
                    .unwrap_or((0, false));
                board.1 = true;
                println!("\tBoard number {} won after {} draws and last drawn number {}\n\tThe solution is {}", i_board, i_seq, n, sum_unmarked.0*n);
            }
        }
    }
}
