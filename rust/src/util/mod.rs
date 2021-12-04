use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn read_input<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename).expect("Error reading input file");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<T>()
                .expect("Could not parse line for provided type")
        })
        .collect()
}

pub fn to_string_vec(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}