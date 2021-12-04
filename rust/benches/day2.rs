use aoc_2021_rust;

use aoc_2021_rust::day2::Move;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day2_part1(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<Move>("inputs/day2.txt");
    c.bench_function("day2 - part1", |b| {
        b.iter(|| aoc_2021_rust::day2::exec_moves(black_box(&input)))
    });
}

pub fn day2_part2(c: &mut Criterion) {
    let input = aoc_2021_rust::util::read_input::<Move>("inputs/day2.txt");
    c.bench_function("day2 - part2", |b| {
        b.iter(|| aoc_2021_rust::day2::exec_moves_with_aim(black_box(&input)))
    });
}

criterion_group!(benches, day2_part1, day2_part2,);
criterion_main!(benches);
