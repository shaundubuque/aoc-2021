#![allow(dead_code)]

use std::collections::HashMap;
use derive_more::Display;
use crate::util::to_string_vec;

const WIN_COUNT: usize = 5;

#[cfg(test)]
mod tests {
    use crate::util::to_string_vec;
    use super::super::util;
    use super::*;

    fn get_sample_input() -> Vec<String> {
        util::read_input::<String>("inputs/day4_sample.txt")
    }

    #[test]
    fn test_selection_parsing() {
        let input = get_sample_input();
        let selections_str = input.first().unwrap();

        let expected_nums = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let selections: Vec<i32> = selections_str
            .split(',')
            .map(|char| char.parse::<i32>().unwrap())
            .collect();

        assert_eq!(expected_nums, selections);
    }

    #[test]
    fn test_input_parsing() {
        let input = get_sample_input();
        let parsed_input = parse_input(input);

        let exp_selections: Vec<String> = to_string_vec(vec![
            "7", "4", "9", "5", "11", "17", "23", "2", "0", "14", "21", "24", "10", "16", "13",
            "6", "15", "25", "12", "22", "18", "20", "8", "19", "3", "26", "1",
        ]);

        let exp_boards: Vec<Vec<String>> = vec![
            to_string_vec(vec!["22 13 17 11  0", " 8  2 23  4 24", "21  9 14 16  7", " 6 10  3 18  5", " 1 12 20 15 19"]),
            to_string_vec(vec![" 3 15  0  2 22", " 9 18 13 17  5", "19  8  7 25 23", "20 11 10 24  4", "14 21 16 12  6"]),
            to_string_vec(vec!["14 21 17 24  4", "10 16 15  9 19", "18  8 23 26 20", "22 11 13  6  5", " 2  0 12  3  7"]),
        ];

        assert_eq!(exp_selections, parsed_input.selections);
        assert_eq!(exp_boards, parsed_input.boards);
    }

    #[test]
    fn test_parse_line_as_row() {
        let line = "22 13 17 11  0".to_string();
        let row = parse_line_as_row(&line);
        assert_eq!(to_string_vec(vec!["22", "13", "17", "11", "0"]), row);

        let line = " 1  3 17 11  0".to_string();
        let row = parse_line_as_row(&line);
        assert_eq!(to_string_vec(vec!["1", "3", "17", "11", "0"]), row);
    }

    #[test]
    fn test_run_sample_game() {
        let input = get_sample_input();
        let game_config = parse_input(input);
        let score = run_game(game_config);
        assert_eq!(4512, score);
    }
}

fn game_state_from_config(config: RawConfig) -> GameState {
    let mut boards: Vec<HashMap<String, (usize,usize)>> = vec![];
    let mut rows: Vec<Vec<usize>> = vec![];
    let mut cols: Vec<Vec<usize>> = vec![];

    for board in config.boards {
        let board_rows = vec![0;5];
        let board_cols = vec![0;5];
        let mut board_map: HashMap<String, (usize, usize)> = HashMap::new();

        for (row_num, line) in board.iter().enumerate() {
            let row = parse_line_as_row(line);
            for (col_num, col) in row.iter().enumerate() {
                board_map.insert(col.clone(), (row_num, col_num));
            }
        }
        boards.push(board_map);
        rows.push(board_rows);
        cols.push(board_cols);
    }
    GameState{ selections: config.selections, boards, rows, cols}
}

fn sum_of_unselected(board: HashMap<String, (usize, usize)>) -> usize {
    board.keys().map(|num_string| num_string.parse::<usize>().unwrap()).sum()
}

pub fn run_game(config: RawConfig) -> usize {

    // Need to build up board state so we can process selections
    let game_state = game_state_from_config(config);
    let selections = game_state.selections;

    let mut boards = game_state.boards;
    let mut rows = game_state.rows;
    let mut cols = game_state.cols;

    // Step through selections, checking each boards row/cols as we go
    let mut winning_number: usize = 0;
    let mut winning_selection:Option<String> = None;
    let mut winning_board: HashMap<String, (usize, usize)> = HashMap::new();
    'game_loop: for selection in selections {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            // check to see if selection exists in map
            let key = selection.as_str();
            if let Some(bingo_counters) = board.get(key) {
                let (r, c) = *bingo_counters;
                if rows[board_idx][r] == WIN_COUNT - 1 || cols[board_idx][c] == WIN_COUNT - 1 {
                    winning_selection = Some(selection.clone());
                } else {
                    rows[board_idx][r] += 1;
                    cols[board_idx][c] += 1;
                }
            }
            board.remove(key);
            if let Some(selection) = winning_selection  {
                winning_number = selection.parse::<usize>().unwrap();
                winning_board = board.clone();
                break 'game_loop
            }
        }
    }
    let board_score = sum_of_unselected(winning_board) * winning_number;
    println!("Score w/winning board: {}", board_score);
    board_score
}

#[derive(Debug, PartialEq, Display)]
#[display(fmt = "\n Selections: {:?} \n Boards: {:?}", selections, boards)]
pub struct RawConfig {
    selections: Vec<String>,
    boards: Vec<Vec<String>>,
}

#[derive(Debug, PartialEq, Display)]
#[display(fmt = "\n Selections: {:?} \n Boards: {:?}", selections, boards)]
struct GameState {
    selections: Vec<String>,
    boards: Vec<HashMap<String, (usize,usize)>>,
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}


fn parse_line_as_row(line: &String) -> Vec<String> {
    to_string_vec(line.split_whitespace().collect())
}

pub fn parse_input(input: Vec<String>) -> RawConfig {
    let mut m_input = input.clone();
    let first_line: Vec<String> = m_input.drain(0..2).collect();
    let selections_str: &String = first_line.first().unwrap();
    let selections: Vec<String> = selections_str
        .split(',')
        .map(|sel| sel.to_string())
        .collect();

    let mut boards: Vec<Vec<String>> = vec![];
    let mut curr_board: Vec<String> = vec![];

    // Assumes we start with first line of first board (using drain(0..2) skips first two lines)
    for line in m_input {
        if line.trim() == "" {
            boards.push(curr_board.clone());
            curr_board = vec![];
        } else {
            curr_board.push(line);
        }
    }
    boards.push(curr_board);

    let parsed_input = RawConfig { selections, boards };
    println!("ParsedInput: {}", parsed_input);
    parsed_input
}
