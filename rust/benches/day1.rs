use aoc_2021_rust;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn read_and_run_part1() {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    aoc_2021_rust::day1::run_part1(black_box(&input));
}

fn read_and_run_part2() {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    aoc_2021_rust::day1::run_part2(black_box(&input));
}

pub fn day1_part1_full(c: &mut Criterion) {
    c.bench_function("day1 - part1 - full", |b| b.iter(|| read_and_run_part1()));
}

pub fn day1_part2_full(c: &mut Criterion) {
    c.bench_function("day1 - part2 - full", |b| b.iter(|| read_and_run_part2()));
}

pub fn day1_part1(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part1", |b| b.iter(|| aoc_2021_rust::day1::run_part1(black_box(&input))));
}

pub fn day1_part2(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part2", |b| b.iter(|| aoc_2021_rust::day1::run_part2(black_box(&input))));
}

criterion_group!(benches, day1_part1, day1_part2, day1_part1_full, day1_part2_full);
criterion_main!(benches);