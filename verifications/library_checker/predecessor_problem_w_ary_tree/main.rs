// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use complete_64_part_tree::Complete64PartTree;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (_n, q) = io.v2::<usize, usize>();
    let mut tree = Complete64PartTree::new(10000000);
    let t = io.digits();
    for (i, ti) in t.iter().enumerate() {
        if *ti == 1 {
            tree.insert(i as u64);
        }
    }
    for _ in 0..q {
        let (c, k) = io.v2::<usize, u64>();
        if c == 0 {
            tree.insert(k);
        } else if c == 1 {
            tree.remove(k);
        } else if c == 2 {
            io.out(usize::from(tree.contains(k)).line())
        } else if c == 3 {
            if tree.contains(k) {
                io.out(k.line())
            } else if let Some(ans) = tree.next(k) {
                io.out(ans.line())
            } else {
                io.out((-1).line())
            }
        } else if c == 4 {
            if tree.contains(k) {
                io.out(k.line())
            } else if let Some(ans) = tree.prev(k) {
                io.out(ans.line())
            } else {
                io.out((-1).line())
            }
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6 9
        010101
        3 3
        4 3
        4 0
        0 4
        1 3
        2 4
        2 3
        3 3
        4 3",
        "3
        3
        -1
        1
        0
        4
        1",
    ));
}
