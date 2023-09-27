//! <https://judge.yosupo.jp/problem/line_add_get_min>

use dynamic_li_chao_tree::DynamicLiChaoTree;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let mut dlct = DynamicLiChaoTree::default();
    for _ in 0..n {
        let (a, b) = io.v2::<i64, i64>();
        dlct.add_line(a, b);
    }
    for _ in 0..q {
        if io.v::<i64>() == 0 {
            let (a, b) = io.v2::<i64, i64>();
            dlct.add_line(a, b);
        } else {
            let p = io.v::<i64>();
            io.out(dlct.query(p).line())
        }
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "2 8
    -1 -1
    0 1
    1 -1
    1 -2
    1 0
    1 2
    0 0 -10
    1 -2
    1 0
    1 2",
        "0
    1
    -1
    -3
    -10
    -10
    -10",
    ))
}
