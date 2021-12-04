#![allow(dead_code)]

use crate::day2::{exec_moves, exec_moves_with_aim, Move};

mod day1;
mod day2;
mod day3;
mod day4;
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

fn main() {
    // run_day_1();
    // run_day2();
    run_day3();
}
