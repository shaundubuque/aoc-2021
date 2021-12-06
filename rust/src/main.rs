#![allow(dead_code)]

use day5::VentLine;
use crate::day2::{exec_moves, exec_moves_with_aim, Move};
use crate::day6::{parse_state, run_eff_simulation, run_simple_simulation, run_simulation};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn run_day_1() {
    let lines = util::read_input::<i32>("inputs/day1.txt");

    let count = day1::run_part1_fold(&lines);
    println!("PART 1 Result: {}", count);

    let count = day1::run_part1_for(&lines);
    println!("PART 1 Result: {}", count);

    let count = day1::run_generalized_zip(&lines, 1);
    println!("PART 1 Result: {}", count);

    let count = day1::run_part2_fold(&lines);
    println!("PART 2 Result: {}", count);

    let count = day1::run_part2_for(&lines);
    println!("PART 2 Result: {}", count);

    let count = day1::run_generalized_zip(&lines, 3);
    println!("PART 2 Result: {}", count);
}

fn run_day2() {
    let moves = util::read_input::<Move>("inputs/day2.txt");
    let sub_location = exec_moves(&moves);
    println!("Sub Location (Part 1): {}", sub_location);

    let moves = util::read_input::<Move>("inputs/day2.txt");
    let sub_location = exec_moves_with_aim(&moves);
    println!("Sub Location (Part 2): {}", sub_location);
}

fn run_day3() {
    let report = util::read_input::<String>("inputs/day3.txt");
    let power_report = day3::get_power_report(&report);
    println!("Power Report {}", power_report);

    let report = util::read_input::<String>("inputs/day3.txt");
    let life_support_rating = day3::get_life_support_rating(&report);
    println!("Life Support Rating {}", life_support_rating);
}

fn run_day4() {
    let input = util::read_input::<String>("inputs/day4.txt");
    let game_config = day4::parse_input(input);
    let score = day4::run_game_part1(game_config);
    println!("PART1 :: Board Score: {}", score);

    let input = util::read_input::<String>("inputs/day4.txt");
    let game_config = day4::parse_input(input);
    let score = day4::run_game_part2(game_config);
    println!("PART2 :: Board Score: {}", score);
}

fn run_day5() {
    let input = util::read_input::<VentLine>("inputs/day5.txt");
    let score = day5::run_part1(input);
    println!("PART1 :: Num Overlapping: {}", score);

    let input = util::read_input::<VentLine>("inputs/day5.txt");
    let score = day5::run_part2(input);
    println!("PART2 :: Num Overlapping: {}", score);
}

fn run_day6() {
    let input = util::read_input::<String>("inputs/day6.txt");
    let init_state = parse_state(input);
    let num_fish = run_simulation(init_state, 80);
    println!("PART1 :: Num Fish: {}", num_fish);

    let input = util::read_input::<String>("inputs/day6.txt");
    let init_state = parse_state(input);
    let num_fish = run_eff_simulation(init_state, 256);
    println!("PART2 :: Num Fish: {}", num_fish);

    let input = util::read_input::<String>("inputs/day6.txt");
    let init_state = parse_state(input);
    let num_fish = run_simple_simulation(init_state, 256);
    println!("PART2 (Simple) :: Num Fish: {}", num_fish);
}

fn main() {
    // run_day_1();
    // run_day2();
    // run_day3();
    // run_day4();
    // run_day5();
    run_day6();
}
