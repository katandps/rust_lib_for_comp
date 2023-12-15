//! # 愚直実装とBinary-GCDの速度の比較

use complib::algebra::binary_operation::greatest_common_divisor::Gcd;
use complib::algebra::Magma;
use complib::algo::xor_shift::XorShift;
use criterion::{criterion_group, criterion_main, Criterion};

criterion_group!(benches, f);
criterion_main!(benches);

fn f(c: &mut Criterion) {
    let mut xorshift = XorShift::default();
    c.bench_function("Naive GCD", |b| {
        b.iter(|| crate::Gcd::op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60)));
    });
    c.bench_function("Binary GCD", |b| {
        b.iter(|| Gcd::op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60)));
    });
}
