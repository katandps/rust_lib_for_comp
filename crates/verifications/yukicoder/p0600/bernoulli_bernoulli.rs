//! # Bernoulli Bernoulli
use rust_lib_for_comp::{
    algebra::{
        lagrange_interpolation::lagrange_polynomical,
        mod_int::{const_mod_factorial::Factorial, ModInt},
    },
    util::io_util::*,
};
use verify::{Solver, Yukicoder};

#[derive(Yukicoder)]
pub struct P0665;
impl verify::Solver for P0665 {
    const PROBLEM_ID: &'static str = "665";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, k) = reader.v2::<usize, usize>();
        let mut v = Vec::new();
        let mut cur = ModInt::zero();
        for i in 0..=k + 1 {
            v.push(cur);
            cur += ModInt::from(i + 1).pow(k as i64);
        }
        let mvt: Factorial<1_000_000_007, 10010> = Factorial::new();
        let ans = lagrange_polynomical(&mvt, &v, n);
        writeln!(write, "{ans}").ok();
    }
}

#[test]
fn test() {
    P0665::assert("10 1", "55");
    P0665::assert("10 3", "3025");
    P0665::assert("10000 10000", "379988108");
    P0665::assert("1234567890123456 10000", "837110143");
}
