use criterion::{Criterion, criterion_group, criterion_main};
use div_pow10::{bit64, bit128};
use std::hint::black_box;

#[inline(never)]
fn bit32_std(a: u32, b: u32) -> u32 {
    a / b
}
#[inline(never)]
fn bit32_std_const(a: u32) -> u32 {
    a / 10000
}

#[inline(never)]
fn bit64_std(a: u64, b: u64) -> u64 {
    a / b
}
#[inline(never)]
fn bit64_std_const(a: u64) -> u64 {
    a / 1000_000
}
#[inline(never)]
fn bit64_std_double(a: u128, b: u64) -> u64 {
    (a / b as u128) as u64
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
fn bit128_std_const(a: u128) -> u128 {
    a / 1000_000_000
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

fn bench_bit32(c: &mut Criterion) {
    let mut group = c.benchmark_group("bit32");

    let n = 10_u32.pow(8) - 1;
    let pow = 10_u32.pow(4);
    group.bench_with_input("std", &(n, pow), |b, i| {
        b.iter(|| black_box(bit32_std(i.0, i.1)))
    });
    group.bench_with_input("std-const", &n, |b, i| {
        b.iter(|| black_box(bit32_std_const(*i)))
    });

    // done
    group.finish();
}

fn bench_bit64(c: &mut Criterion) {
    let mut group = c.benchmark_group("bit64");

    let n = 10_u64.pow(18) - 1;
    let d = 10_u128.pow(30) - 1;
    let i = 6_usize;
    let pow = 10_u64.pow(i as u32);
    group.bench_with_input("std", &(n, pow), |b, i| {
        b.iter(|| black_box(bit64_std(i.0, i.1)))
    });
    group.bench_with_input("std-const", &n, |b, i| {
        b.iter(|| black_box(bit64_std_const(*i)))
    });
    group.bench_with_input("single", &(n, i), |b, i| {
        b.iter(|| black_box(bit64_single(i.0, i.1)))
    });
    group.bench_with_input("double", &(d, i), |b, i| {
        b.iter(|| black_box(bit64_double(i.0, i.1)))
    });
    group.bench_with_input("std-double", &(d, pow), |b, i| {
        b.iter(|| black_box(bit64_std_double(i.0, i.1)))
    });

    // done
    group.finish();
}

fn bench_bit128(c: &mut Criterion) {
    let mut group = c.benchmark_group("bit128");

    let n = 10_u128.pow(30) - 1;
    let i = 12_usize;
    let pow = 10_u128.pow(i as u32);
    group.bench_with_input("std", &(n, pow), |b, i| {
        b.iter(|| black_box(bit128_std(i.0, i.1)))
    });
    group.bench_with_input("std-const", &n, |b, i| {
        b.iter(|| black_box(bit128_std_const(*i)))
    });
    group.bench_with_input("single", &(n, i), |b, i| {
        b.iter(|| black_box(bit128_single(i.0, i.1)))
    });
    group.bench_with_input("double", &(pow - 1, n, i), |b, i| {
        b.iter(|| black_box(bit128_double(i.0, i.1, i.2)))
    });
    group.bench_with_input("double-x", &(pow - 1, n, i), |b, i| {
        b.iter(|| black_box(bit128_double_x(i.0, i.1, i.2)))
    });

    // done
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_bit32(c);
    bench_bit64(c);
    bench_bit128(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
