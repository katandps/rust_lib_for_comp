//! # 愚直実装とBinary-GCDの速度の比較

use criterion::{criterion_group, criterion_main, Criterion};
use rust_lib_for_comp::algebra::binary_operation::greatest_common_divisor::Gcd;
use rust_lib_for_comp::algebra::Magma;
use rust_lib_for_comp::algo::xor_shift::XorShift;

criterion_group!(benches, f);
criterion_main!(benches);

fn f(c: &mut Criterion) {
    let mut xorshift = XorShift::default();
    c.bench_function("Naive GCD", |b| {
        let mut gcd = crate::Gcd::default();
        b.iter(|| gcd.op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60)));
    });
    c.bench_function("Binary GCD", |b| {
        let mut gcd = Gcd::default();
        b.iter(|| gcd.op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60)));
    });
}
