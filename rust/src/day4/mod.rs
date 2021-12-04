#![allow(dead_code)]

use derive_more::Display;

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
        let parsedInput = parse_input(input);

        let exp_selections: Vec<String> = to_string_vec(vec![
            "7", "4", "9", "5", "11", "17", "23", "2", "0", "14", "21", "24", "10", "16", "13",
            "6", "15", "25", "12", "22", "18", "20", "8", "19", "3", "26", "1",
        ]);

        let exp_boards: Vec<Vec<String>> = vec![
            to_string_vec(vec!["22 13 17 11  0", " 8  2 23  4 24", "21  9 14 16  7", " 6 10  3 18  5", " 1 12 20 15 19"]),
            to_string_vec(vec![" 3 15  0  2 22", " 9 18 13 17  5", "19  8  7 25 23", "20 11 10 24  4", "14 21 16 12  6"]),
            to_string_vec(vec!["14 21 17 24  4", "10 16 15  9 19", "18  8 23 26 20", "22 11 13  6  5", " 2  0 12  3  7"]),
        ];

        assert_eq!(exp_selections, parsedInput.selections);
        assert_eq!(exp_boards, parsedInput.boards);
    }
}

#[derive(Debug, PartialEq, Display)]
#[display(fmt = "\n Selections: {:?} \n Boards: {:?}", selections, boards)]
struct ParsedInput {
    selections: Vec<String>,
    boards: Vec<Vec<String>>,
}

fn parse_input(input: Vec<String>) -> ParsedInput {
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
            curr_board.push(line.clone());
        }
    }
    boards.push(curr_board);

    let parsed_input = ParsedInput { selections, boards };
    println!("ParsedInput: {}", parsed_input);
    parsed_input
}
