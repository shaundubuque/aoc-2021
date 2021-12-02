#![allow(dead_code)]

const EMPTY: i32 = -1;

struct DepthCounter {
    prior_depth: i32,
    count: i32,
}

struct DepthHistoryCounter {
    prior_depths: [i32; 3],
    sum: i32,
    count: i32,
}

#[cfg(test)]
mod tests {
    use super::super::util;
    use super::*;

    fn get_sample_input() -> Vec<i32> {
        util::read_input::<i32>("inputs/day1_sample.txt")
    }

    #[test]
    fn test_part1_fold() -> Result<(), String> {
        let sample_input = get_sample_input();
        let sample_count = run_part1_fold(&sample_input);
        assert_eq!(7, sample_count);
        Ok(())
    }

    #[test]
    fn test_part1_for() -> Result<(), String> {
        let sample_input = get_sample_input();
        let sample_count = run_part1_for(&sample_input);
        assert_eq!(7, sample_count);
        Ok(())
    }

    #[test]
    fn test_part2_fold() -> Result<(), String> {
        let sample_input = get_sample_input();
        let sample_count = run_part2_fold(&sample_input);
        assert_eq!(5, sample_count);
        Ok(())
    }

    #[test]
    fn test_part2_for() -> Result<(), String> {
        let sample_input = get_sample_input();
        let sample_count = run_part2_for(&sample_input);
        assert_eq!(5, sample_count);
        Ok(())
    }

    #[test]
    fn test_part1_zip() -> Result<(), String> {
        let sample_input = get_sample_input();
        let sample_count = run_generalized_zip(&sample_input, 1);
        assert_eq!(7, sample_count);
        Ok(())
    }

    #[test]
    fn test_part2_zip() -> Result<(), String> {
        let sample_input = get_sample_input();
        let sample_count = run_generalized_zip(&sample_input, 3);
        assert_eq!(5, sample_count);
        Ok(())
    }
}

pub fn run_part1_fold(lines: &Vec<i32>) -> i32 {
    let acc = DepthCounter {
        prior_depth: i32::MAX,
        count: 0,
    };
    let result = lines
        .iter()
        .fold(acc, |acc, &depth_entry| compare_depth(acc, depth_entry));
    result.count
}

pub fn run_part2_fold(lines: &Vec<i32>) -> i32 {
    let acc = DepthHistoryCounter {
        prior_depths: [EMPTY; 3],
        sum: 0,
        count: 0,
    };
    let result = lines.iter().fold(acc, |acc, &depth_entry| {
        compare_depth_history(acc, depth_entry)
    });
    result.count
}

pub fn run_part1_for(lines: &Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in 1..lines.len() {
        if lines[i] > lines[i - 1] {
            counter += 1;
        }
    }
    counter
}

pub fn run_part2_for(lines: &Vec<i32>) -> i32 {
    // start at index 3 (note, we can easily generalize this to any window length)
    let mut counter = 0;
    for i in 3..lines.len() {
        if lines[i] > lines[i - 3] {
            counter += 1;
        }
    }
    counter
}

pub fn run_generalized_zip(depths: &Vec<i32>, offset: usize) -> i32 {
    depths
        .iter()
        .zip(depths.split_at(offset).1)
        .filter(|(prev, curr)| curr > prev)
        .count() as i32
}

fn compare_depth(mut counter: DepthCounter, depth: i32) -> DepthCounter {
    if depth > counter.prior_depth {
        counter.count += 1;
    }
    counter.prior_depth = depth;
    counter
}

fn shift_depths_left(counter: &mut DepthHistoryCounter, depth: i32) {
    // extracted to make it easier to update to length agnostic solution
    counter.prior_depths[0] = counter.prior_depths[1];
    counter.prior_depths[1] = counter.prior_depths[2];
    counter.prior_depths[2] = depth;
}

fn sum_depths(counter: &DepthHistoryCounter) -> i32 {
    // extracted to make it easier to update to length agnostic solution
    counter.prior_depths[0] + counter.prior_depths[1] + counter.prior_depths[2]
}

fn update_with_depth(mut counter: &mut DepthHistoryCounter, depth: i32) {
    shift_depths_left(&mut counter, depth);
    counter.sum = sum_depths(&counter);
}

fn has_missing_history(counter: &DepthHistoryCounter) -> bool {
    counter.prior_depths[0] == EMPTY
        || counter.prior_depths[1] == EMPTY
        || counter.prior_depths[2] == EMPTY
}

fn compare_depth_history(mut counter: DepthHistoryCounter, depth: i32) -> DepthHistoryCounter {
    if has_missing_history(&counter) {
        // Keep accumulating
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
