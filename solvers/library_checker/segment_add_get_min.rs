/// <https://judge.yosupo.jp/problem/segment_add_get_min>
use dynamic_li_chao_tree::DynamicLiChaoTree;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let mut dlct = DynamicLiChaoTree::default();
    for _ in 0..n {
        let (l, r, a, b) = io.v4::<i64, i64, i64, i64>();
        dlct.add_segment(l..r, a, b);
    }
    for _ in 0..q {
        if io.v::<i64>() == 0 {
            let (l, r, a, b) = io.v4::<i64, i64, i64, i64>();
            dlct.add_segment(l..r, a, b);
        } else {
            let p = io.v::<i64>();
            let ans = dlct.query(p);
            if ans == DynamicLiChaoTree::INF {
                io.out("INFINITY\n");
            } else {
                io.out(ans.line())
            }
        }
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "2 8
        -3 3 -1 -1
        0 7 0 1
        1 -1
        1 -2
        1 0
        1 2
        0 -4 2 0 -10
        1 -2
        1 0
        1 2",
        "0
        1
        -1
        -3
        -10
        -10
        -3",
    ));
    solve(io_debug::IODebug::static_assert(
        "0 1
        1 0",
        "INFINITY",
    ))
}
