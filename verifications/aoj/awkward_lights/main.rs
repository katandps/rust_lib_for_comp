// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1308
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use bit_matrix::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    loop {
        let (m, n, d) = io.v3::<usize, usize, usize>();
        if m == 0 && n == 0 && d == 0 {
            break;
        }
        let s = io.matrix::<usize>(n, m);
        let mut bm = BitMatrix::new(n * m, n * m);
        let mut vec = vec![false; n * m];
        for i in 0..n {
            for j in 0..m {
                bm.set(i * m + j, i * m + j, true);
                for k in 0..n {
                    for l in 0..m {
                        if ((i as i64 - k as i64).abs() + (j as i64 - l as i64).abs()) != d as i64 {
                            continue;
                        }
                        bm.set(i * m + j, k * m + l, true);
                    }
                }
                vec[i * m + j] = true ^ (s[i][j] > 0);
            }
        }
        let result = bm.linear_equation(&vec);
        if let Some(_k) = result {
            io.out(1.line())
        } else {
            io.out(0.line())
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "1 1 1
        1
        2 2 1
        1 1
        1 1
        3 2 1
        1 0 1
        0 1 0
        3 3 1
        1 0 1
        0 1 0
        1 0 1
        4 4 2
        1 1 0 1
        0 0 0 1
        1 0 1 1
        1 0 0 0
        5 5 1
        1 1 1 0 1
        0 1 0 1 0
        1 0 1 0 1
        0 1 0 1 0
        1 0 1 0 1
        5 5 2
        0 0 0 0 0
        0 0 0 0 0
        0 0 1 0 0
        0 0 0 0 0
        0 0 0 0 0
        11 11 3
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 1 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        11 11 3
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 1 1 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0
        13 13 7
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 1 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0",
        "1
        1
        0
        1
        0
        0
        1
        1
        0
        1",
    ))
}
