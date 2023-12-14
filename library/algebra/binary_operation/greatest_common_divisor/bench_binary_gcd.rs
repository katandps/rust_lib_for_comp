//! # 愚直実装とBinary-GCDの速度の比較

use algebra::Magma;
use criterion::{criterion_group, criterion_main, Criterion};
use greatest_common_divisor::naive_impl;
use greatest_common_divisor::Gcd;
use xor_shift::XorShift;

criterion_group!(benches, f);
criterion_main!(benches);

fn f(c: &mut Criterion) {
    let mut xorshift = XorShift::default();
    c.bench_function("Naive GCD", |b| {
        b.iter(|| naive_impl::Gcd::op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60)));
    });
    c.bench_function("Binary GCD", |b| {
        b.iter(|| Gcd::op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60)));
    });
}
