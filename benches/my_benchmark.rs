use criterion::{criterion_group, criterion_main, Criterion};
use adventofcode_2020::days::*;




fn criterion_benchmark(c: &mut Criterion) {
   /* let day1_string = day1::get_input();
    let day2_string = day2::get_input();
    let day3_string = day3::get_input();
    let day4_string = day4::get_input();*/
    let day5_string = day5::get_input();

    /*c.bench_function("day1_1", |b| b.iter(|| day1::day1_1(&day1::input2hash(&day1_string))));
    c.bench_function("day1_1_alt", |b| b.iter(|| day1::day1_1_alt(&day1_string)));
    c.bench_function("day1_2", |b| b.iter(|| day1::day1_2(&day1::input2hash(&day1_string))));
    c.bench_function("day2_1", |b| b.iter(|| day2::day2_1(&day2::string2vec(&day2_string))));
    c.bench_function("day2_2", |b| b.iter(|| day2::day2_2(&day2::string2vec(&day2_string))));
    c.bench_function("day3_1", |b| b.iter(|| day3::day3_1(&day3_string)));
    c.bench_function("day3_2", |b| b.iter(|| day3::day3_2(&day3_string)));
    c.bench_function("day4_1", |b| b.iter(|| day4::day4_1(&day4_string)));
    c.bench_function("day4_2", |b| b.iter(|| day4::day4_2(&day4_string)));*/
    let day5_converted = day5::convert(&day5_string);
    c.bench_function("day5 convert", |b| b.iter(|| day5::convert(&day5_string)));
    c.bench_function("day5_1", |b| b.iter(|| day5::day5_1(&day5_converted)));
    c.bench_function("day5_2", |b| b.iter(|| day5::day5_2(&day5_converted)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);