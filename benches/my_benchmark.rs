/*use criterion::{black_box, criterion_group, criterion_main, Criterion};
use adventofcode_2020::days::day1;



fn criterion_benchmark(c: &mut Criterion) {
    let input_string = day1::get_input();
    let input_hash = day1::input2hash(&day1::get_input());
    let input_vec = day1::input2vec(&day1::get_input());

    c.bench_function("day1_1", |b| b.iter(|| day1::day1_1(&input_hash)));
    c.bench_function("day1_1_old", |b| b.iter(|| day1::day1_1_old(&input_vec)));
    c.bench_function("day1_2", |b| b.iter(|| day1::day1_2(&input_hash)));
    c.bench_function("day1_2_old", |b| b.iter(|| day1::day1_2_old(&input_vec)));
    c.bench_function("converting", |b| b.iter(|| day1::input2hash(&input_string)));
    c.bench_function("day1_1_with_input", |b| b.iter(|| day1::day1_1(&day1::input2hash(&day1::get_input()))));
    c.bench_function("day1_1_old_with_input", |b| b.iter(|| day1::day1_1_old(&day1::input2vec(&day1::get_input()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);*/