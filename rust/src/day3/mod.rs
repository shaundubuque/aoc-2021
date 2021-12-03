#![allow(dead_code)]

use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
#[display(fmt = "\n gamma: {} \n epsilon: {} \n power: {} \n", gamma, epsilon, "self.power()")]
pub struct PowerConsumption {
    gamma: usize,
    epsilon: usize
}

impl PowerConsumption {
    pub fn power(&self) -> usize {
        &self.gamma * &self.epsilon
    }
}

#[derive(Debug, PartialEq, Display)]
#[display(fmt = "\n oxygen rating: {} \n c02 rating: {} \n rating: {} \n", oxygen_rating, co2_rating, "self.rating()")]
pub struct LifeSupportRating {
    oxygen_rating: usize,
    co2_rating: usize
}

impl LifeSupportRating {
    pub fn rating(&self) -> usize {
        &self.oxygen_rating * &self.co2_rating
    }
}

#[cfg(test)]
mod tests {
    use super::super::util;
    use super::*;

    fn get_sample_input() -> Vec<String> {
        util::read_input::<String>("inputs/day3_sample.txt")
    }

    #[test]
    fn test_sample_power_report() {
        let input = get_sample_input();
        let power_report = get_power_report(&input);
        assert_eq!(PowerConsumption {gamma: 22, epsilon: 9}, power_report)
    }

    #[test]
    fn test_sample_life_support_rating() {
        let input = get_sample_input();
        let life_support_rating = get_life_support_rating(&input);
        assert_eq!(LifeSupportRating {oxygen_rating: 23, co2_rating: 10}, life_support_rating)
    }
}

fn most_common_bits(report: &Vec<String>) -> String {
    let mut counters = vec![0; report[0].len()];

    for line in report {
        for(i, bit) in line.chars().enumerate() {
            counters[i] += if bit == '0' {-1} else {1};
        }
    }
    counters.iter().map(|bit_sum| if *bit_sum >= 0 {"1"} else {"0"}).collect()
}

fn oxygen_matcher(report: &Vec<String>, index: usize) -> u8 {
    fn compare(first: i32, second: i32) -> bool {
        return first < second
    }
    matcher_bit_for_index(report, index, '1' as u8, compare)
}

fn co2_matcher(report: &Vec<String>, index: usize) -> u8 {
    fn compare(first: i32, second: i32) -> bool {
        return first > second
    }
    matcher_bit_for_index(report, index, '0' as u8, compare)
}

fn matcher_bit_for_index(report: &Vec<String>, index: usize, tie_breaker: u8, compare: fn(i32, i32) -> bool) -> u8 {
    let mut counter = 0;

    for line in report {
        counter += if line.chars().nth(index).unwrap() == '0' {-1} else {1};
    }
    if counter == 0 { tie_breaker.clone() } else { if compare(counter, 0) {'0' as u8} else {'1' as u8} }
}

fn least_common_bits(most_common_bits: &String) -> String {
    most_common_bits.chars().map(|bit_char| if bit_char == '0' {'1'} else {'0'}).collect()
}

pub fn get_power_report(report: &Vec<String>) -> PowerConsumption {
    let gamma_str = most_common_bits(report);
    let gamma_val = usize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon_val: usize = (2_usize.pow(gamma_str.len() as u32) - gamma_val - 1) as usize;

    return PowerConsumption {gamma: gamma_val, epsilon:epsilon_val}
}

fn find_match(entries: &Vec<String>, match_fn: fn(&Vec<String>, usize) -> u8) -> String {
    let mut working_set = entries.clone();

    for i in 0..entries.first().unwrap().len() {
        let match_bit = match_fn(&working_set, i);
        working_set = working_set.iter().filter(|entry| entry.as_bytes()[i] == match_bit as u8).cloned().collect();
        if working_set.len() == 1 {
            return working_set.first().unwrap().clone();
        }
    }
    panic!("No matching pattern")
}

fn bit_string_as_usize(input: String) -> usize {
    usize::from_str_radix(input.as_str(), 2).unwrap()
}

pub fn get_life_support_rating(report: &Vec<String>) -> LifeSupportRating {
    let oxygen_rating_string = find_match(report, oxygen_matcher);
    let oxygen_rating = bit_string_as_usize(oxygen_rating_string);

    let co2_rating_string = find_match(report, co2_matcher);
    let co2_rating = bit_string_as_usize(co2_rating_string);

    return LifeSupportRating{ oxygen_rating, co2_rating }
}