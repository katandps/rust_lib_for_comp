// verification-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use string_util::*;
use treap::Treap;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let mut map = Treap::default();
    for _ in 0..io.v() {
        if 0 == io.v() {
            let (k, v) = io.v2::<usize, usize>();
            if map.find(&k).is_some() {
                map.remove(&k);
            }
            map.insert(k, v);
        } else {
            let k = io.v();
            io.out(map.find(&k).unwrap_or(&0).line())
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "8
        0 1 2
        1 1
        1 2
        0 2 3
        1 1
        1 2
        0 2 1
        1 2",
        "2
        0
        2
        3
        1",
    ));
}
