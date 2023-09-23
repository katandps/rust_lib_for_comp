//! <https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_B>

use io_util::*;
use primal_dual::PrimalDual;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (v, e, f) = io.v3::<usize, usize, i64>();
    let uvcd = io.vec4::<usize, usize, i64, i64>(e);

    let mut pd = PrimalDual::new(v);
    for (u, v, c, d) in uvcd {
        pd.add_edge(u, v, c, d);
    }
    if let Some(ans) = pd.min_cost_flow(0, v - 1, f) {
        io.out(ans.line());
    } else {
        io.out((-1).line());
    }
    io.flush()
}

#[test]
fn test() {
    let io = io_debug::IODebug::new(
        "4 5 2
        0 1 2 1
        0 2 1 2
        1 2 1 1
        1 3 1 3
        2 3 2 1",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("6");
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
