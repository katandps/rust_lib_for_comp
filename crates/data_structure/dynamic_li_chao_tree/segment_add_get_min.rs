use dynamic_li_chao_tree::DynamicLiChaoTree;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let mut dlct = DynamicLiChaoTree::default();
    for _ in 0..n {
        let (l, r, a, b) = io.v4::<i64, i64, i64, i64>();
        dlct.add_segment(.., l..r, a, b);
    }
    for _ in 0..q {
        if io.v::<i64>() == 0 {
            let (l, r, a, b) = io.v4::<i64, i64, i64, i64>();
            dlct.add_segment(.., l..r - 1, a, b);
        } else {
            let p = io.v::<i64>();
            io.out(dlct.query(p).ln())
        }
    }
    io.flush();
    dbg!(&dlct);
}
