use std::collections::HashSet;
use std::fmt::Error;
use std::str::FromStr;
use crate::util::to_string_vec;

#[derive(Debug, PartialEq)]
pub struct Sequence {
    samples: Vec<String>,
    output: Vec<String>
}

impl FromStr for Sequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("|").collect();

        let samples: Vec<String> = to_string_vec(parts[0].trim().split(' ').collect());
        let output: Vec<String> = to_string_vec(parts[1].trim().split(' ').collect());

        Ok(Sequence {
            samples,
            output
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::util::to_string_vec;
    use super::super::util;
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = util::read_input::<Sequence>("inputs/day8_sample.txt");
        let expected = vec![
            Sequence {
                samples: to_string_vec(vec!["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"]),
                output: to_string_vec(vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"])
            }
        ];
        assert_eq!(expected[0], input[0]);
    }

    #[test]
    fn test_sample_count() {
        let input = util::read_input::<Sequence>("inputs/day8_sample.txt");
        let num_unique_in_output = count_unique_digits(input);
        assert_eq!(26, num_unique_in_output);
    }
}

fn num_unique_digits_in_sequence(seq: &Sequence, unique_digits: &HashSet<usize>) -> usize {
    seq.output.iter().filter(|&val| unique_digits.contains(&(val.chars().count()))).count()
}

pub fn count_unique_digits(sequences: Vec<Sequence>) -> usize {
    let unique_digits: HashSet<usize> = [2, 4, 3, 7].iter().cloned().collect();

    let num_appearances = sequences.iter().fold(0, |acc, seq| acc + num_unique_digits_in_sequence(&seq, &unique_digits));
    num_appearances
}