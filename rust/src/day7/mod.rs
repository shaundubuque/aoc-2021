#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::super::util;
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = util::read_input::<String>("inputs/day7_sample.txt");
        let positions = parse_starting_positions(input);
        println!("Positions: {:?}", positions);
        let expected: Vec<i32> = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(expected,positions);
    }

    #[test]
    fn test_fuel_for_new_position() {
        let input = util::read_input::<String>("inputs/day7_sample.txt");
        let positions = parse_starting_positions(input);
        let fuel = fuel_for_new_position(&positions, &2, constant_cost_for_move);

        assert_eq!(37, fuel);
    }

    #[test]
    fn test_sample_positions_part1() {
        let input = util::read_input::<String>("inputs/day7_sample.txt");
        let positions = parse_starting_positions(input);
        let (position, _) = find_best_position(&positions, constant_cost_for_move);
        assert_eq!(2,position);
    }

    #[test]
    fn test_sample_positions_part2() {
        let input = util::read_input::<String>("inputs/day7_sample.txt");
        let positions = parse_starting_positions(input);
        let (position, _) = find_best_position(&positions, inc_cost_for_move);
        assert_eq!(5,position);
    }

    #[test]
    fn test_cost_for_move() {
        let start_pos = 16;
        let stop_pos = 5;
        let cost = inc_cost_for_move(&start_pos, &stop_pos);
        assert_eq!(66, cost);

        let start_pos = 1;
        let stop_pos = 5;
        let cost = inc_cost_for_move(&start_pos, &stop_pos);
        assert_eq!(10, cost);

        let start_pos = 14;
        let stop_pos = 5;
        let cost = inc_cost_for_move(&start_pos, &stop_pos);
        assert_eq!(45, cost);
    }
}

pub fn parse_starting_positions(input: Vec<String>) -> Vec<i32> {
    let input_line = input.first();
    input_line.unwrap().split(",").map(|input| input.parse::<i32>().unwrap()).collect()
}

pub fn inc_cost_for_move(start: &i32, stop: &i32) -> i32 {
    /* Each change of 1 step in horizontal position costs 1 more unit of fuel than the last:
        the first step costs 1, the second step costs 2, the third step costs 3, and so on.
       Solution: Sum of increasing sequence = n(n + 1)/2 where n = length of sequence
    */
    let n = (start - stop).abs();
    n*(n+1)/2
}

pub fn constant_cost_for_move(start: &i32, stop: &i32) -> i32 {
    (start - stop).abs()
}

pub fn fuel_for_new_position(positions: &Vec<i32>, position: &i32, cost_fn: fn(&i32, &i32) -> i32) -> i32 {
    positions.iter().fold(0, |sum, val| sum + cost_fn(val, position))
}

pub fn find_best_position(starting_positions: &Vec<i32>, cost_fn: fn(&i32, &i32) -> i32) -> (i32, i32) {
    let mut best_position: i32 = starting_positions.first().unwrap().clone();
    let mut least_fuel = i32::MAX;

    let min_position = starting_positions.iter().min().unwrap().clone();
    let max_position = starting_positions.iter().max().unwrap().clone();

    for position in min_position..=max_position {
        let fuel = fuel_for_new_position(&starting_positions, &position, cost_fn);
        if fuel < least_fuel {
            least_fuel = fuel.clone();
            best_position = position.clone()
        };
    }
    (best_position, fuel_for_new_position(&starting_positions, &best_position, cost_fn))
}

