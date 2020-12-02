#![feature(test)]

extern crate test;
use test::Bencher;

use adventofcode_2020::days::day1;

#[bench]
fn day1_1_bench(b: &mut Bencher) {
    let input = day1::input2hash(&day1::get_input());

    b.iter(|| {
        day1::day1_1(&input)
    })
}

#[bench]
fn day1_2_bench(b: &mut Bencher) {
    let input = day1::input2hash(&day1::get_input());

    b.iter(|| {
        day1::day1_2(&input)
    })
}

#[bench]
fn day1_1_old_bench(b: &mut Bencher) {
    let input = day1::input2vec(&day1::get_input());

    b.iter(|| {
        day1::day1_1_old(&input)
    })
}

#[bench]
fn day1_2_old_bench(b: &mut Bencher) {
    let input = day1::input2vec(&day1::get_input());

    b.iter(|| {
        day1::day1_2_old(&input)
    })
}
/*
#[bench]
fn day1_1_bench_with_input(b: &mut Bencher) {
    b.iter(|| {
        day1::day1_1(&day1::input2hash(&day1::get_input()))
    })
}

#[bench]
fn day1_1_old_bench_with_input(b: &mut Bencher) {
    b.iter(|| {
        day1::day1_1_old(&day1::input2vec(&day1::get_input()))
    })
}

#[bench]
fn day1_1_convert_bench(b: &mut Bencher) {
    let input = day1::get_input();
    b.iter(|| {
        day1::input2hash(&input)
    })
}*/