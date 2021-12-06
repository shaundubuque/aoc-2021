#![allow(dead_code)]

use std::fmt::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct VentLine {
    start: (usize, usize),
    stop: (usize, usize)
}

enum LineType {
    HORIZONTAL,
    VERTICAL,
    DIAGONAL
}

impl VentLine {
    fn line_type(&self) -> LineType {
        if self.start.0 == self.stop.0 {
            LineType::VERTICAL
        } else if self.start.1 == self.stop.1 {
            LineType::HORIZONTAL
        } else {
            LineType::DIAGONAL
        }
    }
}

impl FromStr for VentLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("->").collect();
        let start_pair: Vec<&str> = parts[0].trim().split(',').collect();
        let stop_pair: Vec<&str> = parts[1].trim().split(',').collect();

        Ok(VentLine {
            start: (start_pair[0].parse::<usize>().unwrap(), start_pair[1].parse::<usize>().unwrap()),
            stop: (stop_pair[0].parse::<usize>().unwrap(), stop_pair[1].parse::<usize>().unwrap())
        })
    }
}


#[cfg(test)]
mod tests {
    use super::super::util;
    use super::*;

    fn get_sample_input() -> Vec<VentLine> {
        util::read_input::<VentLine>("inputs/day5_sample.txt")
    }

    #[test]
    fn test_input_parsing() {
        let input_line = "0,9 -> 5,9";
        let parsed_input = input_line.parse::<VentLine>().unwrap();

        assert_eq!(VentLine{start: (0,9), stop: (5,9)}, parsed_input);
    }

    #[test]
    fn test_find_max() {
        let line1 = "7,9 -> 5,9".parse::<VentLine>().unwrap();
        let line2 = "0,9 -> 5,12".parse::<VentLine>().unwrap();

        let lines = vec![line1, line2];
        let (max_x, max_y) = get_max_x_y(&lines);

        assert_eq!(7, max_x);
        assert_eq!(12, max_y);
    }

    #[test]
    fn test_part1_sample() {
        let sample_score = run_part1(get_sample_input());
        assert_eq!(5, sample_score);
    }

    #[test]
    fn test_part2_sample() {
        let sample_score = run_part2(get_sample_input());
        assert_eq!(12, sample_score);
    }
}

fn is_h_or_v(vent_line: &VentLine) -> bool {
    vent_line.start.0 == vent_line.stop.0 || vent_line.start.1 == vent_line.stop.1
}

fn get_max_x_y(lines: &Vec<VentLine>) -> (usize, usize) {
    let max_y = lines.iter().fold(usize :: MIN, |max,line| max.max(line.stop.1.max(line.start.1)));
    let max_x = lines.iter().fold(usize :: MIN, |max,line| max.max(line.stop.0.max(line.start.0)));

    (max_x, max_y)
}

fn range_from_pair_inclusive(left: usize, right: usize) -> RangeInclusive<usize> {
    if left < right { left..=right } else { right..=left }
}

pub fn run_part1(lines: Vec<VentLine>) -> usize {
    // filter out diagonal lines
    let hv_lines: Vec<VentLine> = lines.into_iter().filter(|vent_line| is_h_or_v(vent_line)).collect();
    calc_overlapping(hv_lines)
}

pub fn run_part2(lines: Vec<VentLine>) -> usize {
    calc_overlapping(lines)
}

fn calc_overlapping(lines: Vec<VentLine>) -> usize {
    // Find max X / Y
    let (max_x, max_y) = get_max_x_y(&lines);

    // Init Map
    let mut map = vec![vec![0 as usize; max_x + 1]; max_y + 1];

    // Step through lines and increment map
    for line in lines.iter() {
        match line.line_type() {
            LineType::HORIZONTAL=> {
                // increment start_x through stop_x
                let y = line.start.1;
                for x in range_from_pair_inclusive(line.start.0, line.stop.0) {
                    map[y][x] += 1;
                }
            },
            LineType::VERTICAL => {
                // increment start_y through stop_y
                let x = line.start.0;
                for y in range_from_pair_inclusive(line.start.1, line.stop.1) {
                    map[y][x] += 1;
                }
            }
            LineType::DIAGONAL => {
                // increment start_y through stop_y
                let steps = ((line.start.0 as i32) - (line.stop.0 as i32)).abs();

                for i in 0..=steps {
                    let x_step= if line.start.0 < line.stop.0 { i } else { -i };
                    let y_step = if line.start.1 < line.stop.1 { i } else { -i };
                    map[(line.start.1 as i32 + y_step) as usize][(line.start.0 as i32 + x_step) as usize] += 1;
                }
            }
        }
    }

    let num_overlapping = map.iter().flat_map(|array| array.iter()).filter(|count| **count >= 2).count();
    num_overlapping
}