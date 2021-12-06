#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use super::super::util;
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = util::read_input::<String>("inputs/day6_sample.txt");
        let input_line = input.first();

        let init_state: Vec<i32> = input_line.unwrap().split(",").map(|input| input.parse::<i32>().unwrap()).collect();

        let expected: Vec<i32> = vec![3, 4, 3, 1, 2];
        assert_eq!(expected,init_state);
    }

    #[test]
    fn test_simulation() {
        let input = util::read_input::<String>("inputs/day6_sample.txt");
        let init_state = parse_state(input);

        let num_fish = run_simulation(init_state.clone(), 1);
        assert_eq!(5, num_fish);

        let num_fish = run_simulation(init_state.clone(), 2);
        assert_eq!(6, num_fish);

        let num_fish = run_simulation(init_state.clone(), 18);
        assert_eq!(26, num_fish);

        let num_fish = run_simulation(init_state.clone(), 80);
        assert_eq!(5934, num_fish);

        let num_fish = run_simulation(init_state.clone(), 256);
        assert_eq!(26984457539, num_fish);
    }

    #[test]
    fn test_efficient_simulation() {
        let input = util::read_input::<String>("inputs/day6_sample.txt");
        let init_state = parse_state(input);

        let num_fish = run_eff_simulation(init_state.clone(), 1);
        assert_eq!(5, num_fish);

        let num_fish = run_eff_simulation(init_state.clone(), 2);
        assert_eq!(6, num_fish);

        let num_fish = run_eff_simulation(init_state.clone(), 18);
        assert_eq!(26, num_fish);

        let num_fish = run_eff_simulation(init_state.clone(), 80);
        assert_eq!(5934, num_fish);

        let num_fish = run_eff_simulation(init_state.clone(), 256);
        assert_eq!(26984457539, num_fish);
    }
}

pub fn parse_state(input: Vec<String>) -> Vec<usize> {
    let input_line = input.first();
    input_line.unwrap().split(",").map(|input| input.parse::<usize>().unwrap()).collect()
}

// Initial timer for new born fish
const BIRTH_TIMER: usize = 8;

// Reset timer after fish gives birth
const AFTER_BIRTH_TIMER: usize = 6;

enum LanternFishEvent {
    BIRTH,
    NONE
}

struct LanternFish {
    timer: usize
}

impl LanternFish {
    fn tick(&mut self) -> LanternFishEvent {
        match self.timer {
            0 => {
                self.timer = AFTER_BIRTH_TIMER;
                LanternFishEvent::BIRTH
            }
            _ => {
                self.timer -= 1;
                LanternFishEvent::NONE
            }
        }
    }
}

pub fn run_simulation(init_state: Vec<usize>, num_days: usize) -> usize {
    // Create lanternfish for each value in init_state

    let mut fishes: Vec<LanternFish> = init_state.iter().map(|init_timer| LanternFish { timer: *init_timer }).collect();

    for _ in 0..num_days {
        let mut new_batch: Vec<LanternFish> = vec![];
        let num_fishes = fishes.len();
        for i in 0..num_fishes {
            let event = fishes[i].tick();
            match event {
                LanternFishEvent::BIRTH => new_batch.push(LanternFish{ timer: BIRTH_TIMER }),
                _ => {}
            }
        }
        fishes.extend(new_batch);
    }
    fishes.len()
}

pub trait LifecycleMap {
    fn insert_or_increment(&mut self, key: usize, inc_val: usize);
}

impl LifecycleMap for HashMap<usize, usize> {
    fn insert_or_increment(&mut self, key: usize, inc_val: usize) {
        let map = self.clone();
        let entry = map.get(&key);
        match entry {
            Some(val) => self.insert(key, val + inc_val),
            None => self.insert(key, inc_val),
        };
    }
}

pub fn run_eff_simulation(init_state: Vec<usize>, days: usize) -> usize {
    // Create lantern fish for each value in init_state

    let mut lifecycle_groups : HashMap<usize, usize> = HashMap::new();
    for key in init_state {
        lifecycle_groups.insert_or_increment(key, 1);
    }

    for _ in 0..days {
        let mut new_groups : HashMap<usize, usize> = HashMap::new();
        for (key, value) in lifecycle_groups {
            match key {
                0 => {
                    new_groups.insert_or_increment(AFTER_BIRTH_TIMER, value);
                    new_groups.insert_or_increment(BIRTH_TIMER, value);
                },
                x => {
                    new_groups.insert_or_increment(x-1, value);
                }
            }
        }
        lifecycle_groups = new_groups;
    }
    lifecycle_groups.values().sum()
}

