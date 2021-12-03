#![allow(dead_code)]

use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
#[display(fmt = "\n gamma: {} \n epsilon: {} \n power: {} \n", gamma, epsilon, "self.power()")]
pub struct PowerReport {
    gamma: usize,
    epsilon: usize
}

impl PowerReport {
    pub fn power(&self) -> usize {
        &self.gamma * &self.epsilon
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
    fn test_sample_report() {
        let input = get_sample_input();
        let power_report = get_power_report(&input);
        assert_eq!(PowerReport{gamma: 22, epsilon: 9}, power_report)
    }
}

pub fn get_power_report(report: &Vec<String>) -> PowerReport {
    let mut counters = vec![0; report[0].len()];

    for line in report {
        for(i, bit) in line.chars().enumerate() {
            counters[i] += if bit == '0' {-1} else {1};
        }
    }

    let gamma_str: String = counters.iter().map(|bit_sum| if *bit_sum > 0 {"1"} else {"0"}).collect();
    let gamma_val = usize::from_str_radix(gamma_str.as_str(), 2).unwrap();

    let base: usize = 2;
    let epsilon_val: usize = base.pow(counters.len() as u32) - gamma_val - 1;

    return PowerReport{gamma: gamma_val, epsilon:epsilon_val}
}