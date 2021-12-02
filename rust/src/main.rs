mod day1;
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

fn main() {
    run_day_1();
}