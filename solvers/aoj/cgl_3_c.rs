//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/3/CGL_3_C>

use convex_hull::*;
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    for _ in 0..io.v::<usize>() {
        let p = Vector::from(io.v2::<f64, f64>());
        io.out(
            match polygon.include(p) {
                Including::Inside => 2,
                Including::OnLine => 1,
                Including::Outside => 0,
            }
            .line(),
        )
    }
    io.flush();
}

#[test]
fn test() {
    let io = io_debug::IODebug::new(
        "4
        0 0
        3 1
        2 3
        0 3
        3
        2 1
        0 2
        3 2",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "2
                1
                0",
            );
            while let Some(a) = outer.next() {
                if let Some(b) = expect.next() {
                    assert_eq!(a, b, "expect '{}' but actual '{}'", b, a);
                } else {
                    assert_eq!("expect exit but actual {}", a)
                }
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
