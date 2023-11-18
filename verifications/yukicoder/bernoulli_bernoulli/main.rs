// verification-helper: PROBLEM https://yukicoder.me/problems/no/665
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use const_mod_factorial::Factorial;
use io_util::*;
use lagrange_interpolation::lagrange_polynomical;
use mod_int::ModInt;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, k) = io.v2::<usize, usize>();
    let mut v = Vec::new();
    let mut cur = ModInt::zero();
    for i in 0..=k + 1 {
        v.push(cur);
        cur += ModInt::from(i + 1).pow(k as i64);
    }
    let mvt: Factorial<1_000_000_007, 10010> = Factorial::new();
    let ans = lagrange_polynomical(&mvt, &v, n);
    io.out(ans.line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("10 1", "55"));
    solve(io_debug::IODebug::static_assert("10 3", "3025"));
    solve(io_debug::IODebug::static_assert("10000 10000", "379988108"));
    solve(io_debug::IODebug::static_assert(
        "1234567890123456 10000",
        "837110143",
    ));
}
