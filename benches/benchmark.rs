mod binary_gcd;
mod set;

use criterion::criterion_main;
criterion_main!(set::insert, binary_gcd::bench);
