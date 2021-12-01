mod day1;
mod util;

fn run_day_1() {
    let lines = util::read_input::<i32>("inputs/day1.txt");

    let count = day1::run_part1(&lines);
    println!("PART 1 Result: {}", count);

    let count = day1::run_part2(&lines);
    println!("PART 2 Result: {}", count);
}

fn main() {
    run_day_1();
}