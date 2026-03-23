use criterion::{Criterion, criterion_group, criterion_main};
use div_pow10::{bit64, bit128};
use std::hint::black_box;

#[inline(never)]
fn bit64_std(a: u64, b: u64) -> u64 {
    a / b
}
#[inline(never)]
fn bit64_single(a: u64, i: usize) -> u64 {
    unsafe { bit64::unchecked_div_single(a, i) }
}
#[inline(never)]
fn bit64_double(a: u128, i: usize) -> (u64, u64) {
    unsafe { bit64::unchecked_div_double(a, i) }
}

#[inline(never)]
fn bit128_std(a: u128, b: u128) -> u128 {
    a / b
}
#[inline(never)]
fn bit128_single(a: u128, i: usize) -> u128 {
    unsafe { bit128::unchecked_div_single(a, i) }
}
#[inline(never)]
fn bit128_double(a: u128, b: u128, i: usize) -> (u128, u128) {
    unsafe { bit128::unchecked_div_double(a, b, i) }
}
#[inline(never)]
fn bit128_double_x(a: u128, b: u128, i: usize) -> (u128, u128) {
    bit128::x_unchecked_div_double(a, b, i)
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");

    let n = 10_u64.pow(18) - 1;
    let d = 10_u128.pow(30) - 1;
    let i = 6_usize;
    let pow = 10_u64.pow(i as u32);
    group.bench_with_input("bit64-std", &(n, pow), |b, i| {
        b.iter(|| black_box(bit64_std(i.0, i.1)))
    });
    group.bench_with_input("bit64-single", &(n, i), |b, i| {
        b.iter(|| black_box(bit64_single(i.0, i.1)))
    });
    group.bench_with_input("bit64-double", &(d, i), |b, i| {
        b.iter(|| black_box(bit64_double(i.0, i.1)))
    });

    let n = 10_u128.pow(30) - 1;
    let i = 12_usize;
    let pow = 10_u128.pow(i as u32);
    group.bench_with_input("bit128-std", &(n, pow), |b, i| {
        b.iter(|| black_box(bit128_std(i.0, i.1)))
    });
    group.bench_with_input("bit128-single", &(n, i), |b, i| {
        b.iter(|| black_box(bit128_single(i.0, i.1)))
    });
    group.bench_with_input("bit128-double", &(pow - 1, n, i), |b, i| {
        b.iter(|| black_box(bit128_double(i.0, i.1, i.2)))
    });
    group.bench_with_input("bit128-double-x", &(pow - 1, n, i), |b, i| {
        b.iter(|| black_box(bit128_double_x(i.0, i.1, i.2)))
    });

    // done
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    bench(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
