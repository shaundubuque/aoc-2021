use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const EMPTY: i32 = -1;

struct DepthCounter {
    prior_depth: i32,
    count: i32
}

struct DepthHistoryCounter {
    prior_depths: [i32; 3],
    sum: i32,
    count: i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample_input() -> Result<(), String> {
        let sample_count = run_part1("inputs/day1_sample.txt");
        assert_eq!(7, sample_count);
        Ok(())
    }

    #[test]
    fn test_part2_sample_input() -> Result<(), String> {
        let sample_count = run_part2("inputs/day1_sample.txt");
        assert_eq!(5, sample_count);
        Ok(())
    }
}

fn run_part1(filename: &str) -> i32 {
    let depth_lines= read_lines(filename);
    let acc = DepthCounter { prior_depth: EMPTY, count: 0 };
    let result = depth_lines.iter().cloned().fold(acc, |acc, depth_entry| check_depth(acc, parse_entry(depth_entry)));
    result.count
}

fn run_part2(filename: &str) -> i32 {
    let depth_lines= read_lines(filename);
    let acc = DepthHistoryCounter { prior_depths: [EMPTY; 3], sum: 0, count: 0 };
    let result = depth_lines.iter().cloned().fold(acc, |acc, depth_entry| check_depth_history(acc, parse_entry(depth_entry)));
    result.count
}

fn main() {
    let count = run_part1("inputs/day1.txt");
    println!("PART 1 Result: {}", count);

    let count = run_part2("inputs/day1.txt");
    println!("PART 2 Result: {}", count);
}

fn parse_entry(depth_entry: String) -> i32 {
    depth_entry.parse().unwrap_or(EMPTY)
}

fn check_depth(mut counter: DepthCounter, depth: i32) -> DepthCounter {
    if depth > counter.prior_depth {
        counter.count += 1;
    }
    counter.prior_depth = depth;
    counter
}

fn shift_depths_left(counter: &mut DepthHistoryCounter, depth: i32) {
    counter.prior_depths[0] = counter.prior_depths[1];
    counter.prior_depths[1] = counter.prior_depths[2];
    counter.prior_depths[2] = depth;
}

fn sum_depths(counter: &DepthHistoryCounter) -> i32 {
    counter.prior_depths[0] + counter.prior_depths[1] + counter.prior_depths[2]
}

fn update_with_depth(mut counter: &mut DepthHistoryCounter, depth: i32) {
    shift_depths_left(&mut counter, depth);
    counter.sum = sum_depths(&counter);
}

fn check_depth_history(mut counter: DepthHistoryCounter, depth: i32) -> DepthHistoryCounter {
    // doing direct lookup for speed, could use length agnostic filter to support history of arbitrary depth
    if counter.prior_depths[0] == EMPTY || counter.prior_depths[1] == EMPTY || counter.prior_depths[2] == EMPTY {
        // if either first 3 entries are empty, we need to continue accumulating
        update_with_depth(&mut counter, depth);
        counter
    } else {
        let prior_sum = counter.sum;
        update_with_depth(&mut counter, depth);

        if counter.sum > prior_sum {
            counter.count += 1;
        }
        counter
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
    where P: AsRef<Path> {
    let file = File::open(filename).expect("Error reading input file");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
