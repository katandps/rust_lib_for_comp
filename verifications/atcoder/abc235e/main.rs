// verification-helper: PROBLEM https://atcoder.jp/contests/abc235/tasks/abc235_e
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use partially_persistent_union_find::PartiallyPersistentUnionFind;
use slice_bounds::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m, q) = io.v3::<usize, usize, usize>();
    let mut abc = io.vec3::<usize, usize, i64>(m);
    let uvw = io.vec3::<usize, usize, i64>(q);
    let mut uf = PartiallyPersistentUnionFind::new(n);
    abc.sort_by_key(|(_a, _b, c)| *c);
    let mut time = vec![0];
    for (a, b, c) in abc {
        uf.unite(a, b);
        time.push(c);
    }
    for (u, v, w) in uvw {
        let t = time.upper_bound(&w) - 1;
        if uf.same(u, v, t) {
            io.out("No".line());
        } else {
            io.out("Yes".line());
        }
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 6 3
        1 2 2
        2 3 3
        1 3 6
        2 4 5
        4 5 9
        3 5 8
        1 3 1
        3 4 7
        3 5 7",
        "Yes
        No
        Yes",
    ));
    solve(io_debug::IODebug::static_assert(
        "2 3 2
        1 2 100
        1 2 1000000000
        1 1 1
        1 2 2
        1 1 5",
        "Yes
        No",
    ));
}
