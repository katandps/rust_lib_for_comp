use criterion::{criterion_group, Criterion};

use rust_lib_for_comp::{
    algebra::binary_operation::greatest_common_divisor, algebra::Magma, algo::xor_shift::XorShift,
};

fn f(c: &mut Criterion) {
    let mut xorshift = XorShift::default();
    c.bench_function("Naive GCD", |b| {
        b.iter(|| {
            greatest_common_divisor::naive_impl::Gcd::op(
                &xorshift.rand(1 << 60),
                &xorshift.rand(1 << 60),
            )
        });
    });
    c.bench_function("Binary GCD", |b| {
        b.iter(|| {
            greatest_common_divisor::Gcd::op(&xorshift.rand(1 << 60), &xorshift.rand(1 << 60))
        });
    });
}

criterion_group!(bench, f);
