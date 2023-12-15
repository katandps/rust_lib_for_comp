// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2659
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use chinese_remainder_theorem::CRT;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (mut n, m, d) = io.v3::<u64, usize, usize>();
    let a = io.vec::<i64>(m);
    let r = io.matrix::<i64>(d, m);
    let mut ok = true;
    for i in 0..d {
        let mut b = Vec::new();
        let mut mo = Vec::new();
        for (j, &aj) in a.iter().enumerate() {
            if r[i][j] != -1 {
                b.push(r[i][j] as u64);
                mo.push(aj as u64)
            }
        }
        if b.is_empty() {
            continue;
        }
        if let Some((r, m)) = CRT::crt_slice(&b, &mo) {
            if n < r {
                ok = false
            } else {
                n = n - (n - r) % m;
            }
        } else {
            ok = false;
        }
    }
    if ok {
        io.out(n.line())
    } else {
        io.out((-1).line())
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3 3 2
        2 3 4
        1 0 3
        0 2 2",
        "2",
    ));
    solve(io_debug::IODebug::static_assert(
        "3 3 2
        2 3 4
        0 0 0
        0 0 1",
        "-1",
    ));
    solve(io_debug::IODebug::static_assert(
        "3 3 2
        2 3 4
        1 0 3
        -1 -1 -1",
        "3",
    ));
}
