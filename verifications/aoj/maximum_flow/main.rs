//# verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_A
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use dinic::Dinic;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (v, e) = io.v2::<usize, usize>();
    let uvc = io.vec3::<usize, usize, i64>(e);
    let mut dinic = Dinic::new(v);
    for (u, v, c) in uvc {
        dinic.add_edge(u, v, c);
    }
    let ans = dinic.max_flow(0, v - 1);
    io.out(ans.line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 5
        0 1 2
        0 2 1
        1 2 1
        1 3 1
        2 3 2",
        "3",
    ));
}
