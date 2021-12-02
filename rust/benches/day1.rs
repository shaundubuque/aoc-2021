use aoc_2021_rust;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn read_and_run_part1() {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    aoc_2021_rust::day1::run_part1_fold(black_box(&input));
}

fn read_and_run_part2() {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    aoc_2021_rust::day1::run_part2_fold(black_box(&input));
}

pub fn day1_part1_full(c: &mut Criterion) {
    c.bench_function("day1 - part1 - full", |b| b.iter(|| read_and_run_part1()));
}

pub fn day1_part2_full(c: &mut Criterion) {
    c.bench_function("day1 - part2 - full", |b| b.iter(|| read_and_run_part2()));
}

pub fn day1_part1_fold(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part1 (fold)", |b| {
        b.iter(|| aoc_2021_rust::day1::run_part1_fold(black_box(&input)))
    });
}

pub fn day1_part2_fold(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part2 (fold)", |b| {
        b.iter(|| aoc_2021_rust::day1::run_part2_fold(black_box(&input)))
    });
}

pub fn day1_part1_for(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part1 (for)", |b| {
        b.iter(|| aoc_2021_rust::day1::run_part1_for(black_box(&input)))
    });
}

pub fn day1_part2_for(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part2 (for)", |b| {
        b.iter(|| aoc_2021_rust::day1::run_part2_for(black_box(&input)))
    });
}

pub fn day1_part1_zip(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part1 (zip)", |b| {
        b.iter(|| aoc_2021_rust::day1::run_generalized_zip(black_box(&input), 1))
    });
}

pub fn day1_part2_zip(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<i32>("inputs/day1.txt");
    c.bench_function("day1 - part2 (zip)", |b| {
        b.iter(|| aoc_2021_rust::day1::run_generalized_zip(black_box(&input), 3))
    });
}

criterion_group!(
    benches,
    day1_part1_fold,
    day1_part2_fold,
    day1_part1_for,
    day1_part2_for,
    day1_part1_zip,
    day1_part2_zip,
    day1_part1_full,
    day1_part2_full
);
criterion_main!(benches);
