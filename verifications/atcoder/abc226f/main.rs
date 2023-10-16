// verification-helper: PROBLEM https://atcoder.jp/contests/abc226/tasks/abc226_f
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use algebra::Magma;
use const_mod_val_table::ModValTable;
use greatest_common_divisor::Gcd;
use io_util::*;
use mod_int::mod998244353::Mi;
use split_of_natural_number::SplitOfNumber;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, k) = io.v2::<usize, usize>();
    let mut ans = Mi::zero();
    let mvt: ModValTable<_, 10010> = ModValTable::new();
    for p in SplitOfNumber::from(n) {
        let mut score = Mi::from(
            p.iter()
                .fold(1, |a, x| a * *x as i64 / Gcd::op(&a, &(*x as i64))),
        )
        .pow(k as i64)
            * mvt.factorial(n as i64);
        let mut cnt = vec![0; 51];
        for pi in p {
            cnt[pi] += 1;
            score /= pi as i64;
        }
        for cnt in cnt.iter().take(50 + 1).skip(1) {
            if *cnt > 0 {
                score /= mvt.factorial(*cnt);
            }
        }
        ans += score;
    }
    io.out(ans.line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("2 2", "5"));
    solve(io_debug::IODebug::static_assert("3 3", "79"));
    solve(io_debug::IODebug::static_assert("50 10000", "77436607"));
}
