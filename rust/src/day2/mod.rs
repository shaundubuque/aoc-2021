#![allow(dead_code)]

use derive_more::Display;
use std::fmt::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    command: Command,
    value: i32,
}

#[derive(Debug, Display)]
#[display(fmt = "\n x: {} \n aim: {} \n depth: {} \n total: {}", x, aim, depth, "x*depth")]
pub struct SubPosition {
    x: i32,
    aim: i32,
    depth: i32,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let command_fromstr: Command = match parts[0] {
            "forward" => Command::Forward,
            "down" => Command::Down,
            "up" => Command::Up,
            _ => panic!("Error parsing command type"),
        };

        let value_fromstr = parts[1]
            .parse::<i32>()
            .expect("Error parsing command value");

        Ok(Move {
            command: command_fromstr,
            value: value_fromstr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::util;
    use super::*;

    fn get_sample_input() -> Vec<Move> {
        util::read_input::<Move>("inputs/day2_sample.txt")
    }

    #[test]
    fn test_command_parsing() {
        let move_strings = ["forward 4", "down 2", "up 0"];
        let moves = [
            Move {
                command: Command::Forward,
                value: 4,
            },
            Move {
                command: Command::Down,
                value: 2,
            },
            Move {
                command: Command::Up,
                value: 0,
            },
        ];

        for (i, sub_move_str) in move_strings.iter().enumerate() {
            if let Ok(sub_move) = sub_move_str.parse() {
                assert_eq!(moves[i], sub_move);
            } else {
                assert!(false, "Error parsing command");
            }
        }
    }

    #[test]
    fn test_moving() {
        let moves = get_sample_input();
        let sub_location = exec_moves(&moves);
        assert_eq!(sub_location.x, 15);
        assert_eq!(sub_location.depth, 10);
    }

    #[test]
    fn test_moving_with_aim() {
        let moves = get_sample_input();
        let sub_location = exec_moves_with_aim(&moves);
        assert_eq!(sub_location.x, 15);
        assert_eq!(sub_location.depth, 60);
    }
}

pub fn exec_moves(moves: &Vec<Move>) -> SubPosition {
    let mut sub_position = SubPosition { x: 0, aim: 0, depth: 0 };

    for sub_move in moves {
        match sub_move.command {
            Command::Forward => sub_position.x += sub_move.value,
            Command::Down => sub_position.depth += sub_move.value,
            Command::Up => sub_position.depth -= sub_move.value,
        }
    }
    sub_position
}

pub fn exec_moves_with_aim(moves: &Vec<Move>) -> SubPosition {
    let mut sub_position = SubPosition { x: 0, aim: 0, depth: 0 };

    for sub_move in moves {
        match sub_move.command {
            Command::Forward => {
                sub_position.x += sub_move.value;
                sub_position.depth += sub_move.value * sub_position.aim;
            },
            Command::Down => {
                sub_position.aim += sub_move.value;
            },
            Command::Up => {
                sub_position.aim -= sub_move.value
            },
        }
    }
    sub_position
}
